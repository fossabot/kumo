FROM golang:1.12.5-alpine AS builder

ENV GIN_MODE=release

COPY . /app
WORKDIR /app
RUN apk update \
  && apk add git \
  && go build -o /app/sky

ENTRYPOINT ["/app/sky"]
