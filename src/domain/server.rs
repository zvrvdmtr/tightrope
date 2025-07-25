use std::collections::{HashSet};
use serde::Deserialize;

// TODO: нужен ли тут port или достаточно host?
#[derive(Clone, Deserialize)]
pub struct BackendServer {
    pub name: String,
    pub host: String,
}

pub struct ServerPool {
    pub servers: Vec<BackendServer>,
    pub unique_servers: HashSet<String>,
}

impl ServerPool {
    pub fn new() -> Self {
        ServerPool {
            servers: vec![],
            unique_servers: HashSet::new(),
        }
    }

    // Rewrite server by its key. Because it is simple enough.
    pub fn add_servers(&mut self, servers: Vec<BackendServer>) {
        for server in servers {
            if !self.unique_servers.insert(format!("{}-{}", server.name.clone(), server.host.clone())) {
                panic!(
                    "Server with host {} meets multiple times in config",
                    server.host
                );
            }
            self.servers.push(server);
        }
        println!("{:?}", self.unique_servers)
    }

    pub fn get_all_servers(&self) -> &[BackendServer] {
        &self.servers
    }
}
