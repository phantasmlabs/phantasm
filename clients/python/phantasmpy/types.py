import json
from typing import Any


class HeartbeatResponse:
    version: str

    def __init__(self, version: str):
        self.version = version

    def __str__(self):
        return f"version: {self.version}"


class GetApprovalResponse:
    """Response returned by the get approval method.

    Properties:
    - approved: True if the operation is approved, False otherwise.
    - parameters: Parameters to be used in the operation.
    """

    approved: bool
    parameters: Any

    def __init__(self, approved: bool, parameters: str):
        self.approved = approved
        self.parameters = json.loads(parameters) if parameters else None

    def __str__(self):
        displays = [
            f"approved: {self.approved}",
            f"parameters: {self.parameters}",
        ]

        return "\n".join(displays)
