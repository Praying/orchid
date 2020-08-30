use orchid_proto::proto::raft_pb::raft_pb_server::RaftPb;
use orchid_proto::proto::raft_pb::{
    raft_pb_client::RaftPbClient, raft_pb_server::RaftPbServer, VoteRequest, VoteResponse,
};
use std::mem::MaybeUninit;

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
        //let channel = Endpoint::from_static(format!("http://{}:{}",&self.address,&self.port).as_str());
        //self.client = Box::new(channel);
        Ok(())
    }
}
#[derive(Default)]
pub struct RaftService {}

#[tonic::async_trait]
impl RaftPb for RaftService {
    async fn vote(&self, _reqeust: Request<VoteRequest>) -> Result<Response<VoteResponse>, Status> {
        let reply = VoteResponse { num: 20 };

        Ok(Response::new(reply))
    }
}

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
fn test_create_server() {
    // let addr = "[::1]:50051".parse().unwrap();
    // let raft_service = RaftPbServer::new(RaftService::default());
    // std::thread::spawn(|| {
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    //     let channel = aw!(Endpoint::from_static("http://[::1]:50051").connect()).unwrap();
    //     let mut raft_client = RaftPbClient::new(channel.clone());
    //     let request = tonic::Request::new(VoteRequest {});
    //     let response = aw!(raft_client.vote(request)).unwrap();
    //     println!("vote response = {:?}", response.get_ref().num);
    // });
    // aw!(Server::builder().add_service(raft_service).serve(addr));
}

#[test]
fn test_create_client() {}
