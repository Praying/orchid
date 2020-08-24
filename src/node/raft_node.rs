use futures::task::SpawnExt;
use orchid_proto::proto::raft_pb::raft_pb_server::RaftPb;
use orchid_proto::proto::raft_pb::{
    raft_pb_client::RaftPbClient, raft_pb_server::RaftPbServer, VoteRequest, VoteResponse,
};
use std::mem::MaybeUninit;
use std::net::SocketAddr;
use tonic::{transport::Endpoint, transport::Server, Request, Response, Status};

enum State {
    Leader,
    Candidate,
    Follower,
}

pub struct RaftNode {
    address: String,
    port: u32,
    state: State,
    client: Box<MaybeUninit<Endpoint>>,
    server: Box<MaybeUninit<Endpoint>>,
}

impl RaftNode {
    fn new(address: String, port: u32) -> Self {
        RaftNode {
            address,
            port,
            state: State::Follower,
            client: Box::new_uninit(),
            server: Box::new_uninit(),
        }
    }
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let channel = Endpoint::from_static(format!("http://{}:{}",&self.address,&self.port).as_str());
        // self.client = Box::new(channel);
        Ok(())
    }
}

pub struct MyRaftPB {}

#[tonic::async_trait]
impl RaftPb for MyRaftPB {
    async fn vote(&self, _reqeust: Request<VoteRequest>) -> Result<Response<VoteResponse>, Status> {
        let reply = VoteResponse {};
        Ok(Response::new(reply))
    }
}
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}
#[test]
fn test_create_server() {
    let channel = tonic::transport::Endpoint::from_static("");
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let raft_service = MyRaftPB {};
}

#[test]
fn test_create_client() {
    let mut node = RaftNode::new("127.0.0.1".into(), 8089);
    aw!(node.start());
}
