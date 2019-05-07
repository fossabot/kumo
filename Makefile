PROTO_PATH = ./shared/protobuf

PROTO_OUT_SKY = ./sky/src/proto
PROTO_OUT_VAPOR = ./vapor/src/proto

compute-service:
	protoc --plugin=protoc-gen-rust-grpc --proto_path=./shared/protobuf ./shared/protobuf/compute.proto --rust_out=${PROTO_OUT_SKY}
	protoc --plugin=protoc-gen-rust-grpc --proto_path=./shared/protobuf ./shared/protobuf/compute.proto --rust_out=${PROTO_OUT_VAPOR}

rust-grpc-dependecies:
	cargo install --force grpc-compiler protobuf-codegen
