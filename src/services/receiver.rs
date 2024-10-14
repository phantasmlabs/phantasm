use super::*;
use protos::receiver_server::Receiver as ProtoReceiver;

pub struct Receiver {}

impl Receiver {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl ProtoReceiver for Receiver {
    async fn heartbeat(
        &self,
        _request: Request<()>,
    ) -> Result<Response<protos::HeartbeatResponse>, Status> {
        let response = protos::HeartbeatResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        Ok(Response::new(response))
    }
}
