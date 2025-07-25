use crate::domain::server::BackendServer;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum BalancingAlgorithm {
    RoundRobin,
}

#[derive(Deserialize)]
pub struct Proxy {
    pub balancing_algorithm: BalancingAlgorithm,
}

#[derive(Deserialize)]
pub struct Config {
    pub proxy: Proxy,
    pub servers: Vec<BackendServer>,
}
