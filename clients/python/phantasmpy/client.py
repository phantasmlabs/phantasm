import grpc
from google.protobuf.empty_pb2 import Empty
from stubs import receiver_pb2 as protos
from stubs.receiver_pb2_grpc import ReceiverStub


class Phantasm:
    def __init__(self, host: str = "localhost", port: int = 2505):
        channel = grpc.insecure_channel(f"{host}:{port}")
        self.connection = ReceiverStub(channel)

    def heartbeat(self):
        return self.connection.Heartbeat(request=Empty())

    def get_approval(self):
        pass
