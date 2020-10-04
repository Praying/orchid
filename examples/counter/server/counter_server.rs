use orchid_proto::proto::raft_pb::raft_pb_server::RaftPb;
use orchid_proto::proto::raft_pb::{
    raft_pb_client::RaftPbClient, raft_pb_server::RaftPbServer, VoteRequest, VoteResponse,
};
use std::mem::MaybeUninit;

use orchid::node::raft_node::RaftService;
use tonic::{transport::Endpoint, transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50056".parse().unwrap();
    let raft_service = RaftPbServer::new(RaftService::default());

    Server::builder()
        .add_service(raft_service)
        .serve(addr)
        .await?;
    Ok(())
}
