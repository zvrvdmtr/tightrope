mod balancer;
mod domain;
mod strategies;

use std::{fs, net::SocketAddr, sync::Arc, time::SystemTime};

use balancer::load_balancer::LoadBalancer;
use domain::server::ServerPool;
use http_body_util::Full;
use hyper::{Request, body::Bytes, server::conn::http1, service::service_fn};
use hyper_rustls::HttpsConnector;
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioIo},
};
use strategies::round_robin::RoundRobinStrategy;
use tokio::net::TcpListener;
use toml;

use crate::{
    domain::config::{BalancingAlgorithm, Config},
    strategies::strategy::Strategy,
};

// TODO уменьшить boilerplate и растащить все по функциям или пакетам
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("../config.toml").expect("Should read file.");
    let config: Config = toml::from_str(&contents).unwrap();

    let mut pool = ServerPool::new();
    pool.add_servers(config.servers);

    let connector = build_connector()?;

    let http_client: Arc<Client<_, Full<Bytes>>> =
        Arc::new(Client::builder(TokioExecutor::new()).build(connector));

    let strat = choose_strategy(config.proxy.balancing_algorithm);

    let load_balancer = Arc::new(LoadBalancer::new(strat, http_client, pool));
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 3000))).await?;

    // TODO: подумать над реализацией sticky session
    loop {
        let (stream, _) = listener.accept().await?;
        let load_balancer = Arc::clone(&load_balancer);

        println!("New connection started at {:?}", SystemTime::now());

        tokio::spawn(async move {
            // TODO: req никак не используется
            let service = service_fn(move |mut _req: Request<hyper::body::Incoming>| {
                // Need to clone for every http query
                let load_balancer = Arc::clone(&load_balancer);
                async move { load_balancer.redirect_query().await }
            });

            let io = TokioIo::new(stream);

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("{}", err)
            }
            println!("Connection closed at {:?}", SystemTime::now());
        });
    }
}

fn build_connector() -> Result<
    HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
    Box<dyn std::error::Error>,
> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()?
        .https_or_http()
        .enable_http1()
        .build();
    Ok(https)
}

fn choose_strategy(balancing_algorithm: BalancingAlgorithm) -> impl Strategy + 'static {
    match balancing_algorithm {
        BalancingAlgorithm::RoundRobin => RoundRobinStrategy::new(),
    }
}
