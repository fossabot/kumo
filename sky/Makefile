PROTOC = ../shared/bin/protoc

PROTO_PATH = ./proto

dev:
	go run main.go

build:
	go build

test:
	go test

clean:
	rm sky

protoc:
	$(PROTOC) --proto_path=./proto ./proto/compute.proto --go_out=plugins=grpc:./grpc/compute

proto-dependecies:
	go get -u github.com/golang/protobuf/protoc-gen-go
