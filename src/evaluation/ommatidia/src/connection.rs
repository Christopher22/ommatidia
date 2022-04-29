use std::io::Read;

use hyper::{
    body::Buf,
    client::conn::{self, SendRequest},
    Body, Request, StatusCode,
};
use tokio::{net::TcpStream, task::JoinHandle};
use tower::ServiceExt;

use super::Engine;

#[derive(Debug)]
pub struct Connection(SendRequest<Body>, JoinHandle<()>);

pub enum Error {
    Transmission,
    Http,
}

impl Connection {
    pub async fn new(engine: &Engine, port: u16) -> Result<Self, Error> {
        let stream = TcpStream::connect((engine.host(), port))
            .await
            .or(Err(Error::Transmission))?;
        let (sender, connection) = conn::handshake(stream).await.or(Err(Error::Http))?;
        // Read process in the background
        let background_process = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error in connection: {}", e);
            }
        });
        Ok(Connection(sender, background_process))
    }

    pub async fn send(
        &mut self,
        request: Request<Body>,
    ) -> Result<(StatusCode, impl Read), hyper::Error> {
        // Wait until the connection is ready
        self.0.ready().await?;

        let response = self.0.send_request(request).await?;
        let status_code = response.status();
        let data = hyper::body::aggregate(response.into_body()).await?.reader();
        Ok((status_code, data))
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.1.abort();
    }
}
