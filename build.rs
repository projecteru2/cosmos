fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &[],
        input: &["core.proto"],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc-rust-grpc");
}
