# API Reference

Phantasm offers a simple gRPC API for the client to interact with the server,
specifically the _Receiver Server_. There is only one core API method that you
need to understand and that is the get approval method.

## GetApproval

This is the core method that you will integrate into your AI agent to obtain
approval from the approvers before executing a tool. This is a blocking call, so
if possible, you may want to run it in a separate thread or process.

One of Phantasm's most distinctive features is its ability to pass parameters
back and forth. This allows the approvers to review the parameters that the AI
agent intends to use and either approve or deny the request based on them. The
approvers can also modify the parameters before granting approval.

Ideally, the AI agent should use the parameters returned by the approvers as the
final parameters for executing the tool, rather than the original parameters.
This ensures that the AI agent executes the tool accurately.

### Request

```proto
message GetApprovalRequest {
    string name = 1;
    string parameters = 2;
    string context = 3;
}
```

- name: Name of the function the AI agent wants to execute.
- parameters: Parameters to pass to the function.
- context: Additional context to pass to the approvers.

### Response

```proto
message GetApprovalResponse {
    bool approved = 1;
    string parameters = 2;
}
```

- approved: Boolean indicating if the request was approved.
- parameters: Parameters to pass to the function if approved.
