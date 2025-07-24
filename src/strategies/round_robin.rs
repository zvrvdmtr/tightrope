use core::{
    option::Option::{self, None},
    sync::atomic::{AtomicUsize, Ordering},
    usize,
};

use super::strategy::Strategy;
use crate::domain::server::{Server, ServerPool};

pub struct RoundRobinStrategy {
    index: AtomicUsize,
    pool: ServerPool,
}

impl RoundRobinStrategy {
    pub fn new(pool: ServerPool) -> Self {
        return RoundRobinStrategy {
            index: AtomicUsize::new(0),
            pool: pool,
        };
    }
}

// Think about to change from `Option` to `Result`
impl Strategy for RoundRobinStrategy {
    fn get_next_server(&self) -> Option<&Server> {
        let servers = self.pool.get_all_servers();
        if servers.is_empty() {
            return None;
        }

        let idx = self.index.fetch_add(1, Ordering::Relaxed);
        servers.get(idx % servers.len())
    }
}
