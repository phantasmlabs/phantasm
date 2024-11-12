# Introducing Phantasm

![Demo Dashboard](https://phantasm-assets.s3.amazonaws.com/demos/0.1.0.gif)

Thank you for considering using Phantasm. Phantasm is a simple human-in-the-loop
(HITL) approval layer for AI agents. With Phantasm, you can build and deploy AI
agents that can execute critical business tasks safely.

Most of the information you need to get started can be found in the
[Repository][repo]. This documentation, instead, will focus on explaining the
details and concepts in the project. If you have any questions or need help,
please feel free to reach out to us on [Discord][discord].

[repo]: https://github.com/phantasmlabs/phantasm
[discord]: https://discord.gg/dgevsYhh7P

## Components

Phantasm consists of 3 main components: Client, Server, and Dashboard.

### Server

The server is the primary component that connects the client and the dashboard.
The server responsible for receiving the approval requests from the client is
called the _Receiver Server_ and the server responsible for coordinating the
approval requests with the dashboard is called the _Coordinator Server_.

The _Receiver Server_ uses gRPC while the _Coordinator Server_ uses WebSocket
protocol. Both servers are written in Rust and are controlled by Phantasm's CLI.
If you're using Docker, this CLI is already the primary entry point for the
image.

### Client

The client is a language-specific library that you can use to connect your AI
agent to the _Receiver Server_. The core concept of the client is to send an
approval request to the server before your AI agent executes a certain function
and waits for approval from the approvers.

### Dashboard

Phantasm comes with a web-based dashboard that you can use to connect to the
_Coordinator Server_ and monitor the incoming approval requests from your AI
agents. You or your team can then approve, modify, or reject these requests.
