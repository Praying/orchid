use orchid_proto::proto::raft_pb::raft_pb_server::RaftPb;
use orchid_proto::proto::raft_pb::{
    raft_pb_client::RaftPbClient, raft_pb_server::RaftPbServer, VoteRequest, VoteResponse,
};
use std::mem::MaybeUninit;

use tonic::{transport::Endpoint, transport::Server, Request, Response, Status};
use orchid::node::raft_node::RaftService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let channel = Endpoint::from_static("http://[::1]:50056").connect().await?;
    let mut raft_client = RaftPbClient::new(channel.clone());
    let request = tonic::Request::new(VoteRequest {});
    let response = raft_client.vote(request).await?;
    println!("vote response = {:?}", response.get_ref().num);
    Ok(())
}
