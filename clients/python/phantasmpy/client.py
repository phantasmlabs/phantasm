import grpc
import json
from typing import Any, Dict
from google.protobuf.empty_pb2 import Empty
from .stubs import receiver_pb2 as protos
from .stubs.receiver_pb2_grpc import ReceiverStub
from .types import HeartbeatResponse, GetApprovalResponse


class Phantasm:
    """Create a Phantasm client to interact with the receiver service.

    Args:
    - host: Hostname of the receiver service.
    - port: Port where the receiver listens for requests.
    - secret: Key used to authenticate the client with the receiver.
    """

    def __init__(
        self,
        host: str = "localhost",
        port: int = 2505,
        secret: str = "",
    ):
        channel = grpc.insecure_channel(f"{host}:{port}")
        self.connection = ReceiverStub(channel)
        self.metadata = [("authorization", secret)]

    def heartbeat(self) -> HeartbeatResponse:
        """Check if the client can connect to the receiver service."""

        response = self.connection.Heartbeat(
            request=Empty(),
            metadata=self.metadata,
        )

        return HeartbeatResponse(version=response.version)

    def get_approval(
        self,
        name: str,
        parameters: Any,
        context: str = "",
    ) -> GetApprovalResponse:
        """Request approval for an operation from the approver.

        Args:
        - name: Name of the operation, typically, the function name.
        - parameters: Parameters used in the operation.
        - context: Additional information about the operation.

        Parameters will be relayed as a JSON string to the approver. The
        approver can choose to modify the parameters with the correct values
        before approving the request.
        """

        try:
            request = protos.GetApprovalRequest(
                name=name,
                parameters=json.dumps(parameters),
                context=context,
            )
        except Exception as e:
            raise ValueError(f"Invalid parameters: {e}")

        response = self.connection.GetApproval(
            request=request,
            metadata=self.metadata,
        )

        return GetApprovalResponse(
            approved=response.approved,
            parameters=response.parameters or "",
        )


def test_get_approval():
    params = {
        "x": 5,
        "y": 10,
    }

    phantasm = Phantasm()
    response = phantasm.get_approval(
        name="multiply",
        parameters=params,
        context="Multiply two numbers: x and y.",
    )

    if response.approved:
        result = response.parameters["x"] * response.parameters["y"]
        print("Request Approved")
        print(f"Result: {result}")
    else:
        print("Request Rejected")
