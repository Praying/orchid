fn main() {
    // tonic_build::compile_protos("proto/counter/counter.proto").unwrap();
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["counter/counter.proto"], &["counter"])
        .unwrap();
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["raft/raft.proto"], &["raft"])
        .unwrap();
}
