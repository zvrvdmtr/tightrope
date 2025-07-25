use http_body_util::Full;
use hyper::{Response, StatusCode, Uri, body::Bytes};
use hyper_util::client::legacy::{Client, connect::Connect};
use std::{str::FromStr, sync::Arc};

use crate::{domain::server::{BackendServer, ServerPool}, strategies::strategy::Strategy};

// TODO LoadBalancer должен иметь pool серверов по которому осуществляется балансировка
// TODO подумать, как заменить тип Full<Bytes> на дженерик и стоит ли это делать
pub struct LoadBalancer<C> {
    strategy: Box<dyn Strategy>,
    http_client: Arc<Client<C, Full<Bytes>>>,
    pool: ServerPool,
}

impl<C> LoadBalancer<C>
where
    C: Clone + Send + Sync + Connect + 'static,
{
    pub fn new(strategy: impl Strategy + 'static, client: Arc<Client<C, Full<Bytes>>>, pool: ServerPool) -> Self {
        LoadBalancer {
            strategy: Box::new(strategy),
            http_client: client,
            pool: pool
        }
    }

    pub fn choose_backend(&self) -> Option<BackendServer> {
        self.strategy.get_next_server(&self.pool)
    }

    // TODO: проработать коды ошибок
    pub async fn redirect_query(&self) -> Result<Response<Full<Bytes>>, hyper::http::Error> {
        let backend = match self.choose_backend() {
            Some(backend) => backend,
            None => {
                return Ok(Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Full::new(Bytes::from("No backend available")))
                    .unwrap());
            }
        };

        let uri = match Uri::from_str(&backend.host) {
            Ok(uri) => uri,
            Err(err) => {
                return Ok(Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Full::new(Bytes::from(err.to_string())))
                    .unwrap());
            }
        };

        // TODO не обрабатываем resp
        match self.http_client.get(uri).await {
            // No need Ok() here, body returns Result<> already
            Ok(_resp) => Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from("TODO"))),
            Err(err) => {
                return Ok(Response::builder()
                    .status(StatusCode::SERVICE_UNAVAILABLE)
                    .body(Full::new(Bytes::from(err.to_string())))
                    .unwrap());
            }
        }
    }
}
