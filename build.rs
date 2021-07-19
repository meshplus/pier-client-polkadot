fn main() {
    let paths = vec!["pb/basic.proto","pb/ibtp.proto","pb/plugin.proto"];

    protoc_rust_grpc::Codegen::new()
        .out_dir("src/proto")
        .inputs(IntoIterator::into_iter(paths))
        .include("pb")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
