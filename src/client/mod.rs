pub mod base_url;
pub mod store;

use self::{base_url::MadomeBaseUrl, store::AuthStore};

pub struct MadomeClient {
    base_url: MadomeBaseUrl,
    token: AuthStore,
}

impl MadomeClient {
    pub fn stable() -> Self {
        Self {
            base_url: MadomeBaseUrl::stable(),
            token: Default::default(),
        }
    }

    pub fn beta() -> Self {
        Self {
            base_url: MadomeBaseUrl::beta(),
            token: Default::default(),
        }
    }

    pub fn nightly() -> Self {
        Self {
            base_url: MadomeBaseUrl::nightly(),
            token: Default::default(),
        }
    }
}

macro_rules! impl_madome_client {
    ($($namespace:ident),*$(,)?) => {
        impl MadomeClient {
            $(
                pub fn $namespace(&self) -> $namespace {
                    $namespace {
                        base_url: self.base_url.$namespace.as_str(),
                        token: &self.token
                    }
                }
            )*
        }

        $(
            #[allow(incorrect_ident_case, non_camel_case_types)]
            pub struct $namespace<'a> {
                pub(crate) base_url: &'a str,
                pub(crate) token: &'a dyn $crate::api::TokenBehavior,
            }
        )*
    };
}

impl_madome_client![user, auth];

#[cfg(test)]
pub mod tests {
    use std::{convert::Infallible, fmt::Debug, net::SocketAddr, sync::Arc, time::Duration};

    use http::{Request, Response, StatusCode};
    use hyper::{
        body::HttpBody,
        service::{make_service_fn, service_fn},
        Body, Server,
    };
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use tokio::sync::{oneshot, Mutex};

    use super::MadomeClient;

    impl MadomeClient {
        pub fn e2e_channel(&self, base_url: impl Into<String>) -> e2e_channel {
            e2e_channel {
                base_url: base_url.into(),
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub struct e2e_channel {
        pub base_url: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct Authcode {
        pub code: String,
    }

    #[derive(Serialize)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    enum E2eChannelCommand {
        Authcode { email: String, port: u16 },
    }

    impl E2eChannelCommand {
        pub fn serialize(&self) -> Vec<u8> {
            serde_json::to_vec(self).unwrap()
        }
    }

    impl e2e_channel {
        pub async fn authcode(&self, email: impl Into<String>) -> Authcode {
            let port = available_port();

            let (tx, rx) = oneshot::channel::<Authcode>();

            tokio::spawn(async move {
                open_receiver(tx, port).await;
            });

            let command = E2eChannelCommand::Authcode {
                email: email.into(),
                port,
            };

            let resp = reqwest::Client::new()
                .post(&self.base_url)
                .body(command.serialize())
                .send()
                .await
                .unwrap();

            if resp.status() != StatusCode::NO_CONTENT {
                panic!("e2e-channel is not normal")
            }

            let authcode = rx.await.unwrap();

            authcode
        }
    }

    pub fn available_port() -> u16 {
        for port in [31111, 31112, 31113, 31114].into_iter().cycle() {
            let listener = std::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port)));

            if let Ok(_listener) = listener {
                return port;
            }

            std::thread::sleep(Duration::from_millis(500));
        }

        unreachable!()
    }

    async fn open_receiver<T>(json_tx: oneshot::Sender<T>, port: u16)
    where
        T: Debug + DeserializeOwned + Send + Sync + 'static,
    {
        let (close_tx, close_rx) = oneshot::channel::<()>();

        let close_tx = Arc::new(Mutex::new(Some(close_tx)));
        let json_tx = Arc::new(Mutex::new(Some(json_tx)));

        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        let svc = |close_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
                   json_tx: Arc<Mutex<Option<oneshot::Sender<T>>>>| async move {
            Ok::<_, Infallible>(service_fn(move |request| {
                handle_request(close_tx.clone(), json_tx.clone(), request)
            }))
        };

        let server = hyper::Server::bind(&addr).serve(make_service_fn(move |_| {
            svc(close_tx.clone(), json_tx.clone())
        }));

        let server = Server::with_graceful_shutdown(server, async {
            close_rx.await.ok();
        });

        server.await.ok();
    }

    async fn handle_request<T>(
        close_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
        json_tx: Arc<Mutex<Option<oneshot::Sender<T>>>>,
        mut request: Request<Body>,
    ) -> Result<Response<Body>, Infallible>
    where
        T: Debug + DeserializeOwned + Send + Sync,
    {
        let mut close_tx = close_tx.lock().await;
        close_tx.take().unwrap().send(()).ok();

        let body = request.body_mut();

        let mut buf = Vec::new();

        while let Some(Ok(a)) = body.data().await {
            buf.push(a)
        }

        let buf = buf.concat();

        let json: T = serde_json::from_slice(&buf).expect("json serialize");

        let mut json_tx = json_tx.lock().await;
        json_tx.take().unwrap().send(json).ok();

        Ok::<_, Infallible>(Response::new(Body::empty()))
    }
}
