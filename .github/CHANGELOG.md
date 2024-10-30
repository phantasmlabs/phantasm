# Phantasm Change Logs

This document lists the changes to Phantasm with each release ordered by the
latest release first. Since Phantasm project consists of multiple independent
projects (e.g., server, dashboard, client libraries), the version of the release
will use the date of the release in the format of **YYYY.MM.DD**.

<!-- Add release notes below this line. -->

### 2024.10.29

This is the initial release of Phantasm which includes 3 components: Server,
Dashboard, and Python Client. The server and dashboard are available as Docker
images via GitHub Container Registry. The Python client is available on PyPI.

These are the features available in this release:

- Client can send approval requests to the server via gRPC.
- Server can relay the approval requests to the dashboard via WebSocket.
- Dashboard can displays the approval requests to the approvers.
- Dashboard can send back an approval response to the server via WebSocket.
- Server can send the decision back to the client via gRPC.
