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
        let message = ApprovalRequest::from(request.into_inner());
        let approval_id = message.id;
        tracing::info!("An approval request is created: {approval_id}");

        // Create a one-time use WebSocket channel to coordinate the
        // approval response from the approver. The sender is used to send
        // the approval response from our general WebSocket receiver to
        // the oneshot receiver.
        let (os_sender, os_receiver) = oneshot::channel::<ApprovalResponse>();

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

        let approver = &result.approver.name;
        let verdict = match result.approved {
            true => "approved",
            false => "rejected",
        };

        tracing::info!("The request {approval_id} is {verdict} by {approver}");
        self.reduce_load(&connection_id);
        Ok(Response::new(result.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_heartbeat() {
        let phantasm = Phantasm::open().unwrap();
        let service = Arc::new(phantasm);

        let request = Request::new(());
        let response = service.heartbeat(request).await.unwrap();

        let version = env!("CARGO_PKG_VERSION");
        assert_eq!(response.get_ref().version, version);
    }
}
