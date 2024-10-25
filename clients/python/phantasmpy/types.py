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
    - status: Status of the approval request.
    - parameters: Parameters to be used in the operation.

    Status can be one of: APPROVED, MODIFIED, or DENIED

    If the status is MODIFIED, the parameters will be returned as a Python
    type depending on the modification made by the approver. Otherwise, the
    parameters will be None.
    """

    status: str
    parameters: Any

    def __init__(self, status: str, parameters: str):
        self.status = status
        self.parameters = json.loads(parameters) if parameters else None

    def __str__(self):
        displays = [
            f"status: {self.status}",
            f"parameters: {self.parameters}",
        ]

        return "\n".join(displays)
