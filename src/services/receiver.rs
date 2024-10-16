use super::*;
use protos::receiver_server::Receiver;

#[tonic::async_trait]
impl Receiver for Arc<Phantasm> {
    async fn heartbeat(
        &self,
        _request: Request<()>,
    ) -> Result<Response<protos::HeartbeatResponse>, Status> {
        let response = protos::HeartbeatResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_approval(
        &self,
        request: Request<protos::GetApprovalRequest>,
    ) -> Result<Response<protos::GetApprovalResponse>, Status> {
        unimplemented!()
    }
}
