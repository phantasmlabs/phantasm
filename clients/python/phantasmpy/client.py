import grpc
import json
from typing import Any, Dict
from google.protobuf.empty_pb2 import Empty
from stubs import receiver_pb2 as protos
from stubs.receiver_pb2_grpc import ReceiverStub
from .types import HeartbeatResponse, GetApprovalResponse


class Phantasm:
    """Create a Phantasm client to interact with the receiver service.

    Args:
    - host: Hostname of the receiver service.
    - port: Port where the receiver listens for requests.
    """

    def __init__(self, host: str = "localhost", port: int = 2505):
        channel = grpc.insecure_channel(f"{host}:{port}")
        self.connection = ReceiverStub(channel)

    def heartbeat(self) -> HeartbeatResponse:
        """Check if the client can connect to the receiver service."""

        response = self.connection.Heartbeat(request=Empty())
        return HeartbeatResponse(version=response.version)

    def get_approval(self, name: str, parameters: Any) -> GetApprovalResponse:
        """Request approval for an operation from the approver.

        Args:
        - name: Name of the operation, typically, the function name.
        - parameters: Parameters used in the operation.

        Parameters will be relayed as a JSON string to the approver. The
        approver can choose to modify the parameters with the correct values
        before approving the request.
        """

        try:
            _params = json.dumps(parameters)
            request = protos.GetApprovalRequest(name=name, parameters=_params)
        except Exception as e:
            raise ValueError(f"Invalid parameters: {e}")

        response = self.connection.GetApproval(request=request)
        approved = response.approved
        parameters = response.parameters or ""

        return GetApprovalResponse(approved=approved, parameters=parameters)


def emulate_get_approval():
    def multiply(x: int, y: int):
        return x * y

    params = {
        "x": 5,
        "y": 10,
    }

    phantasm = Phantasm()
    response = phantasm.get_approval(name="multiply", parameters=params)

    if response.approved:
        # Here, we're going to trust our approver and unpack the parameters.
        result = multiply(**response.parameters)

        print("Request Approved")
        print(f"Result: {result}")
    else:
        print("Request Rejected")
