# Configuration

Phantasm starts with a sensible default configuration that you can override
either by setting environment variables or by passing command-line arguments.
This documentation will guide you through the available configuration options.

## Receiver Authentication

Phantasm allows you to configure authentication for the communication between
the client and the Receiver Server. Phantasm uses a simple secret key mechanism
to authenticate the client, and by default, authentication is disabled.

To enable authentication, simply set the `PHANTASM_SECRET` environment variable
to your secret key before starting the server. If you are running Phantasm in a
Docker container, this is how you can set the secret key:

```bash
docker run -e PHANTASM_SECRET=secretkey ...
```

When authentication is enabled, you must provide the secret key when
initializing the client to ensure that the client can authenticate and
communicate with the Receiver Server.

=== "Python"

    ```python
    from phantasmpy import Phantasm
    phantasm = Phantasm(secret="secretkey")
    ```
