mod domain;
mod strategies;
mod balancer;

use domain::server::{Server, ServerPool};
use balancer::load_balancer::LoadBalancer;
use strategies::round_robin::RoundRobinStrategy;

fn main() {
    let mut pool = ServerPool::new();
    let servers = vec![
        Server::new(String::from("localhost"), String::from("127.0.0.1"), 8080),
        Server::new(String::from("first server"), String::from("0.0.0.1"), 8080),
        Server::new(String::from("second server"), String::from("0.0.0.1"), 8081),
    ];
    pool.add_servers(servers);
    for server in pool.get_all_servers() {
        println!("{}", server.name)
    }

    let load_balancer = LoadBalancer::new(RoundRobinStrategy::new(pool));
    load_balancer.serve().unwrap()
}
