package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/foolin/goview/supports/ginview"
	"github.com/gin-gonic/gin"
	"google.golang.org/grpc"

	pb "sky/grpc/compute"
)

var (
	port      = flag.String("port", "8080", "The port that sky will listen for HTTP requests")
	vaporAddr = flag.String("vapor-addr", "127.0.0.1:10100", "The server address in the format of host:port")
)

func main() {
	flag.Parse()

	conn, err := grpc.Dial(*vaporAddr, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	log.Printf("successfully connected to vapor at %s", *vaporAddr)
	defer conn.Close()

	client := pb.NewComputeServiceClient(conn)

	router := gin.Default()
	router.HTMLRender = ginview.Default()

	router.GET("/", func(c *gin.Context) {
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		defer cancel()
		response, err := client.List(ctx, &pb.ComputeListRequest{})
		if err != nil {
			log.Fatalf("%v.ComputeListRequest(_) = _, %v", client, err)
		}

		computes := response.GetComputes()

		//render with master
		c.HTML(http.StatusOK, "index", gin.H{
			"title":    "Home",
			"computes": computes,
		})
	})

	// Listen and Server in 0.0.0.0:8080
	log.Printf("Listening on :%s", *port)
	addr := fmt.Sprintf(":%s", *port)
	router.Run(addr)
}
