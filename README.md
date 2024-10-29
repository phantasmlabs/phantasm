[![GitHub](https://tinyurl.com/5xetszfe)][github]
[![Discord](https://tinyurl.com/5fv83u2h)][discord]
[![License](https://tinyurl.com/mrxn8fvz)][license]

[discord]: https://discord.gg/dgevsYhh7P
[github]: https://github.com/phantasmlabs/phantasm
[license]: https://github.com/phantasmlabs/phantasm/blob/main/LICENSE

![Phantasm](https://phantasm-assets.s3.amazonaws.com/banners/0.1.0.png)

Phantasm offers open-source toolkits that allows you to create human-in-the-loop
(HITL) workflows for modern AI agents. Phantasm comes with 3 main components
that work together to create a seamless HITL experience:

- **Server**: Coordinating the HITL workflows between humans and AI agents.
- **Dashboard**: For the human team to monitor and manage the workflows.
- **Client**: A library to integrate the workflows into your AI agents.

## Features

- ✅ Fully open-source and free to use
- ✅ Works out of the box with any AI framework or model
- ✅ Load balancer to distribute the requests to multiple approvers (Beta)
- ✅ Web-based dashboard to manage the approval workflows
- ✅ Easy-to-use client libraries for popular programming languages

## How It Works

Phantasm allows you to have an approval layer on top of your AI agents. This
means, you're free to use any AI framework or model you see fit. By using
Phantasm, you can delay the execution of certain business critical actions by
your AI agents until a human approves them.

This is particularly useful in scenarios where the AI agent is not 100% accurate
or the cost of making a mistake is high. For example, an AI agent that can
automate calendar scheduling. In this case, you can use Phantasm to make sure
that the AI agent schedules the right meetings at the right time.

Here's how it works under the hood:

1. Your AI agent sends an approval request to Phantasm's server.
2. Phantasm relays the request to a human approver.
3. The approver reviews the request and either approves or rejects it.
4. Phantasm sends the decision back to the AI agent.

Phantasm can also relay the action parameters to the approvers. This parameters
can be modified by the approvers before approving the action. This is useful if
there is only a slight mistake in the action parameters that can be corrected by
the approvers. This way the action can be approved with some modifications
instead of rejected.

![Demo](https://phantasm-assets.s3.amazonaws.com/demos/0.1.0.gif)

## Getting Started

The easiest way to get started with Phantasm is by using Docker. There are 2
components that you need to run before you can start using Phantasm: **Server**
and **Dashboard**. To run these components, first, you need to pull the Docker
images of these components:

```bash
# Replace $TAG with the version of Phantasm you want to use.
docker pull ghcr.io/phantasmlabs/phantasm/dashboard:$TAG
docker pull ghcr.io/phantasmlabs/phantasm/server:$TAG
```

After pulling the images, you can run the components with these commands:

```bash
# Replace $TAG with the version of Phantasm you pulled.
docker run -d -p 2515:2515 ghcr.io/phantasmlabs/phantasm/dashboard:$TAG
docker run -d -p 2505:2505 -p 2510:2510 ghcr.io/phantasmlabs/phantasm/server:$TAG start
```

After running the components, you will have access to:

- **Dashboard**: [localhost:2515](http://localhost:2515)
- **Receiver Server**: localhost:2505
- **Coordinator Server**: localhost:2510

You can use the dashboard to manage the approval workflows by adding a
connection to Phantasm's Coordinator Server. The Receiver Server is used to
receive the approval requests from the AI agents via Phantasm's client library.
Depending on the programming language you use, you can choose the appropriate
client library from the list below:

- [Python](https://pypi.org/project/phantasmpy)

## Contributing

The easiest way to contribute to Phantasm is by starring the repository and
sharing it with your colleagues. By doing so, you help us reach a wider audience
and grow the community. If you are interested in contributing to the project in
the form of code, design, or documentation, please read our
[Contributing Guidelines](.github/CONTRIBUTING.md).

Phantasm is licensed under the GNU General Public License v3.0. By contributing
to the project, you agree to license your contributions under the same license.
For more information, please read our [License](LICENSE).

## Disclaimer

Phantasm is still in its early stages of development. Therefore, use it at your
own risk. With that being said, we are actively working on improving the
toolkits and adding essential features to make it more robust and secure. We
will be happy if you actually use it and provide us with feedback to make it
better.
