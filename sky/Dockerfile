FROM golang:1.12.5 AS builder

ENV GIN_MODE=release

COPY . /app
RUN cd /app \
  && go build -o /usr/bin/sky \
  && cd / \
  && rm -rf /app

ENTRYPOINT ["/usr/bin/sky"]

# FROM alpine

# ENV GIN_MODE=release
# COPY --from=builder /usr/bin/sky /usr/bin/sky

# ENTRYPOINT ["/usr/bin/sky"]
