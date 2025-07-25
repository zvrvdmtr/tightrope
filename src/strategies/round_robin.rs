use core::{
    option::Option::{self, None},
    sync::atomic::{AtomicUsize, Ordering},
};

use super::strategy::Strategy;
use crate::domain::server::{BackendServer, ServerPool};

pub struct RoundRobinStrategy {
    index: AtomicUsize,
}

impl RoundRobinStrategy {
    pub fn new() -> Self {
        return RoundRobinStrategy {
            index: AtomicUsize::new(0),
        };
    }
}

// Think about to change from `Option` to `Result`
impl Strategy for RoundRobinStrategy {
    fn get_next_server(&self, pool: &ServerPool) -> Option<BackendServer> {
        let servers = pool.get_all_servers();
        if servers.is_empty() {
            return None;
        }

        let idx = self.index.fetch_add(1, Ordering::Relaxed);
        servers.get(idx % servers.len()).cloned()
    }
}
