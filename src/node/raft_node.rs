use orchid_proto::proto::raft_pb::raft_pb_server::RaftPb;
use orchid_proto::proto::raft_pb::{
    raft_pb_client::RaftPbClient, raft_pb_server::RaftPbServer, VoteRequest, VoteResponse,
};

use crate::node::raft_option::RaftOptions;
use rand::prelude::*;
use std::collections::HashMap;
use tonic::{transport::Endpoint, transport::Server, Request, Response, Status};

use time::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub enum RaftState {
    StateFollower,
    StateCandidate,
    StateLeader,
}

impl std::default::Default for RaftState {
    fn default() -> RaftState {
        RaftState::StateFollower
    }
}

type NodeID = u64;
// term <-> node
// e.g. (2, 3) represents that  vote for node 3 at term 2
type VoteData = (u64, Option<NodeID>);

#[derive(Default)]
pub struct Raft {
    ///number of ticks since it reached last electionTimeout when it is leader
    ///or candidate.
    /// number of ticks since it reached last electionTimeout or received a
    /// valid message from current leader when it is a follower.
    election_elapsed: u64,
    election_timeout: u64,
    election_random_timeout: u64,
    heartbeat_elapsed: u64,
    heartbeat_timeout: u64,
    heartbeat_random_timeout: u64,
    vote_elapsed: u64,
    vote_timeout: u64,
    vote: VoteData,
    ///current term
    term: u64,
    /// raft state
    state: RaftState,
    ///id to self
    id: NodeID,
}

impl Raft {
    /// create new raft instance
    fn new() -> Self {
        Raft::default()
    }

    ///return  raft node is leader or not
    fn is_leader(&self) -> bool {
        return self.state == RaftState::StateLeader;
    }
    /// become follower
    fn become_follower(&mut self) {
        self.state = RaftState::StateFollower
    }
    /// become candidate
    fn become_candidate(&mut self) {
        self.state = RaftState::StateCandidate
    }
    /// become leader
    fn become_leader(&mut self) {
        self.state = RaftState::StateLeader
    }
    ///past election timeout
    fn past_election_timeout(&self) -> bool {
        self.election_elapsed >= self.election_timeout
    }
    ///past heartbeat timeout
    fn past_heartbeat_timeout(&self) -> bool {
        self.heartbeat_elapsed >= self.heartbeat_timeout
    }

    fn reset_election_timeout(&mut self) {
        let mut r = StdRng::seed_from_u64(time::Time::now().nanosecond() as u64);
        self.election_random_timeout =
            self.election_timeout + r.gen_range(0, self.election_timeout);
    }

    fn reset_heartbeat_timeout(&mut self) {
        let mut r = StdRng::seed_from_u64(time::Time::now().nanosecond() as u64);
        self.heartbeat_random_timeout =
            self.heartbeat_timeout + r.gen_range(0, self.heartbeat_timeout);
    }

    fn set_state(&mut self, state: RaftState) {
        self.state = state;
    }
}

#[test]
fn test_create_raft() {
    let rf = Raft::new();
    assert_eq!(rf.state, RaftState::StateFollower);
}

// impl RaftNode {
//     fn new(address: String, port: u32) -> Self {
//         RaftNode {
//             address,
//             port,
//             state: State::Follower,
//             client: Box::new_uninit(),
//             server: Box::new_uninit(),
//         }
//     }
//     async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         //let channel = Endpoint::from_static(format!("http://{}:{}",&self.address,&self.port).as_str());
//         //self.client = Box::new(channel);
//         Ok(())
//     }
// }
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
fn test_start_raft() {
    let raft_option = RaftOptions::new();
    let mut raft = Raft::new();
    raft.heartbeat_timeout = raft_option.heartbeat_timeout;
    raft.election_timeout = raft_option.election_timeout;
    raft.reset_election_timeout();
    raft.reset_heartbeat_timeout();
    raft.set_state(RaftState::StateFollower);
}

#[test]
fn test_gen_random_num() {
    for i in 1..5 {
        let mut r = StdRng::seed_from_u64(time::Time::now().nanosecond() as u64);
        let x = r.gen_range(0, 300);
        println!("{}", x);
    }
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



#[tokio::test]
async fn test_tokio_timer() {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
    for _i in 1..10{
        interval.tick().await;
        println!("hello {}", _i);
    }
}

#[tokio::test]
async fn test_raft_election_timer(){

    let raft_option = RaftOptions::new();
    let mut raft = Raft::new();
    raft.heartbeat_timeout = raft_option.heartbeat_timeout;
    raft.election_timeout = raft_option.election_timeout;
    raft.id = raft_option.id;
    raft.reset_election_timeout();
    raft.reset_heartbeat_timeout();
    raft.set_state(RaftState::StateFollower);

    // tokio::spawn(async{
    //     loop{
    //         let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
    //         interval.tick().await;
    //         if raft.past_election_timeout(){
    //             raft.become_candidate();
    //         }
    //
    //     }
    // }).await;



}