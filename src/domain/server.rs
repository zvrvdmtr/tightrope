use std::collections::HashMap;

#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: i32,
}

impl Server {
    pub fn new(name: String, host: String, port: i32) -> Self {
        Server {
            name: name,
            host: host,
            port: port,
        }
    }
}

pub struct ServerPool {
    pub servers: Vec<Server>,
    pub unique_servers: HashMap<String, ()>,
}

impl ServerPool {
    pub fn new() -> Self {
        ServerPool {
            servers: vec![],
            unique_servers: HashMap::new(),
        }
    }

    // Rewrite server by its key. Because it is simple enough.
    pub fn add_servers(&mut self, servers: Vec<Server>) {
        for server in servers {
            if self
                .unique_servers
                .contains_key(&format!("{}:{}", server.host, server.port))
            {
                panic!(
                    "Server with host {}:{} meets multiple times in config",
                    server.host, server.port
                )
            }
            self.servers.push(server)
        }
    }

    pub fn get_all_servers(&self) -> &[Server] {
        &self.servers
    }
}
