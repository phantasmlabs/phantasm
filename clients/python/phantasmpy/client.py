import grpc
import json
from typing import Any, Callable
from functools import wraps
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

    def require_approval(self, with_parameters: bool = True) -> Callable:
        """Decorator to request approval before executing a function.

        By default, the decorator will use the parameters provided by the
        approvers to call the function. If you want to use the original
        parameters, set `with_parameters` to false.

        This decorator raises exceptions if:
        - The approval request is rejected.
        - Error when requesting approval from the server.

        Example:

        ```py
        phantasm = Phantasm()
        @phantasm.require_approval()
        def double(x: int) -> int:
            return x * 2

        double(x=5)
        ```
        """

        def decorator(function: Callable) -> Callable:
            @wraps(function)
            def wrapper(**kwargs) -> Callable:
                name = function.__name__
                docs = function.__doc__ or ""
                response = self.get_approval(
                    name=name,
                    parameters=kwargs,
                    context=docs,
                )

                if response.approved:
                    if with_parameters:
                        kwargs = response.parameters
                    return function(**kwargs)

                raise PermissionError("The approval request is not approved.")

            return wrapper

        return decorator


def test_get_approval():
    phantasm = Phantasm()

    @phantasm.require_approval()
    def multiply(x: int, y: int) -> int:
        """Multiply two numbers, x and y."""
        return x * y

    print(multiply(x=5, y=10))
