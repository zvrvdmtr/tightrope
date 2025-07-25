use crate::domain::server::{BackendServer, ServerPool};

pub trait Strategy: Send + Sync {
    fn get_next_server(&self, pool: &ServerPool) -> Option<BackendServer>;
}

