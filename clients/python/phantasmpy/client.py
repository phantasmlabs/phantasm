import grpc
from stubs.receiver_pb2_grpc import ReceiverStub


class Phantasm:
    def __init__(self, host: str = "localhost", port: int = 2505):
        channel = grpc.insecure_channel("localhost:50051")
        self.connection = ReceiverStub(channel)

    def heartbeat(self):
        response = self.connection.Heartbeat()
        return response

    def get_approval(self):
        pass
