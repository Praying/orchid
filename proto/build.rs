use std::path::Path;
fn main() {
    let path = std::path::Path::new("src/proto");
    if path.exists(){
        tonic_build::configure()
            .out_dir(&path)
            .compile(&["counter/counter.proto","raft/raft.proto"], &["counter", "raft"])
            .unwrap();

    }

}
