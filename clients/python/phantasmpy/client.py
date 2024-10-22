import grpc
import json
from typing import Any, Dict
from google.protobuf.empty_pb2 import Empty
from stubs import receiver_pb2 as protos
from stubs.receiver_pb2_grpc import ReceiverStub

STATUS_TO_STRING: Dict[int, str] = {
    0: "APPROVED",
    1: "MODIFIED",
    2: "DENIED",
}


class Phantasm:
    def __init__(self, host: str = "localhost", port: int = 2505):
        channel = grpc.insecure_channel(f"{host}:{port}")
        self.connection = ReceiverStub(channel)

    def heartbeat(self):
        return self.connection.Heartbeat(request=Empty())

    def get_approval(
        self, name: str, parameters: Dict[Any, Any]
    ) -> Dict[str, Any]:
        try:
            _params = json.dumps(parameters)
            request = protos.GetApprovalRequest(name=name, parameters=_params)
        except Exception as e:
            raise ValueError(f"Invalid parameters: {e}")

        response = self.connection.GetApproval(request=request)
        result = {"status": STATUS_TO_STRING[response.status]}
        if response.parameters:
            result["parameters"] = json.loads(response.parameters)

        return result


def emulate_get_approval():
    def multiply(x: int, y: int):
        return x * y

    params = {
        "x": 5,
        "y": 10,
    }

    phantasm = Phantasm()
    response = phantasm.get_approval(name="multiply", parameters=params)

    if response["status"] == "APPROVED":
        print("Request Approved")
        print(f"Result: {multiply(**params)}")
    elif response["status"] == "MODIFIED":
        print("Request Modified")
        result = multiply(**response["parameters"])
        print(f"Result: {result}")
    else:
        print("Request Denied")
