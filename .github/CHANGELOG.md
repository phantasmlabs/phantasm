# Phantasm Change Logs

This document lists the changes to Phantasm with each release ordered by the
latest release first. Since Phantasm project consists of multiple independent
projects (e.g., server, dashboard, client libraries), the version of the release
will use the date of the release in the format of **YYYY.MM.DD**.

To stay consistent with each release, when creating a new release on GitHub, we
use the version format above as the tag name and the release title. The release
notes should be added to the release description in the following format:

```md
Release Note Content

### Contributors

- @username
- @username

### Full Changelog

GitHub Generated Changelog URL
```

<!-- Add release notes below this line. -->

### 2024.11.05

This release includes the following changes:

- Added context parameter to the approval request Protobuf message.
- Added a dedicated documentation website for Phantasm using MkDocs.
- Added initial unit tests for the server component.
- Fixed spelling error in the approval request card in the dashboard.

Documentation: [docs.phantasmlabs.com](https://docs.phantasmlabs.com)

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
