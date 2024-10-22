use super::*;
use protos::receiver_server::Receiver;
use tokio::sync::oneshot;

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
        let connection_id = match self.get_lightest_connection() {
            Some(id) => id,
            None => {
                let message = "No approver is available at the moment";
                return Err(Status::unavailable(message));
            },
        };

        // Unwrap is safe because the connection ID is guaranteed to exist.
        let connection = self.get_connection(&connection_id).unwrap();

        let request = request.into_inner();
        let approval_id = ApprovalID::new();
        tracing::info!("Approval request is created: {approval_id}");

        // Create a oneshot channel to coordinate the approval request
        // and response with the approver.
        let (os_sender, os_receiver) = oneshot::channel::<ApprovalResponse>();

        let message = ApprovalRequest {
            id: approval_id,
            name: request.name,
            parameters: request.parameters,
        };

        // Send the serialized approval request to the approver.
        let message = serde_json::to_string(&message).unwrap();
        if connection.sender.send(Message::Text(message)).is_err() {
            let message = "Failed to send the message to the approver";
            return Err(Status::unavailable(message));
        }

        self.add_load(&connection_id);
        self.add_approval(approval_id, os_sender);

        // Wait for the approval response from the approver.
        let result = os_receiver.await.map_err(|_| {
            Status::internal("Failed to receive approval response")
        })?;

        tracing::info!("Approval request {approval_id}: {:?}", result.status);
        self.reduce_load(&connection_id);

        let response = protos::GetApprovalResponse {
            status: protos::ApprovalStatus::from(result.status) as i32,
            parameters: result.parameters,
        };

        return Ok(Response::new(response));
    }
}
