pub struct RaftOptions {
    pub election_timeout: u64,
    pub heartbeat_timeout: u64,
    /// define one tick equals xx ms
    /// timer runs on tick
    pub tick: u64,

    /// raft node id which points to self
    pub id: u64,
}

impl RaftOptions {
    pub(crate) fn new() -> Self {
        RaftOptions {
            election_timeout: 300,
            heartbeat_timeout: 30,
            tick: 2,
            id: 1,
        }
    }
}
