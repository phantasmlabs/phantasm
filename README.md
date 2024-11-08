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
images of these components.

These Docker images are hosted on GitHub Container Registry:
[Packages](https://github.com/orgs/phantasmlabs/packages?repo_name=phantasm)

```bash
docker pull ghcr.io/phantasmlabs/phantasm/dashboard:latest
docker pull ghcr.io/phantasmlabs/phantasm/server:latest
```

After pulling the images, you can run the components with these commands:

```bash
docker run -d -p 2515:2515 ghcr.io/phantasmlabs/phantasm/dashboard:latest
docker run -d -p 2505:2505 -p 2510:2510 ghcr.io/phantasmlabs/phantasm/server:latest start
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

## Best Practices

### Error Handling & Fallback Mechanism

Phantasm is designed for real-time approval workflows for AI agents with several
external non-technical dependencies. Therefore, it is important to have a good
error handling and fallback mechanism in place. Here are some edge cases that
you should consider before using Phantasm in production:

- No approver is available to monitor and approve the action.
- The approver is not able to approve the action within a certain time frame.
- The approver rejects the action from being executed by the AI agent.

To handle these edge cases, you can use the following strategies:

- Set a timeout for the approval request based on the urgency of the action.
- Create a fall-back action if the approval request is rejected or timed out.
- Retry the approval request if the approver is not available.
- Notify an approver via email if no approver is available at the time.
- Other strategies that are specific to your use case...

### Parameters & Context

When sending an approval request using Phantasm, you can also send the action
parameters and context to be relayed to the approvers. Most of the time, you
want to use this feature to help the approvers make an informed decision.
Parameters are usually the input values of the action while the context is the
additional information that can help the approvers understand the action better.

Here are some best practices when sending parameters and context:

- Send the parameters generated by the AI agent as a JSON object.
- When reviewing the action, make sure the parameters are correct and valid.
- Send the action documentation with parameters explanation in the context.
- Optionally, include the end-user input prompt in the context.

## Contributing

The easiest way to contribute to Phantasm is by starring the repository and
sharing it with your colleagues. By doing so, you help us reach a wider audience
and grow the community. If you are interested in contributing to the project in
the form of code, design, or documentation, please read our
[Contributing Guidelines](.github/CONTRIBUTING.md).

Other than the clients, which is licensed under the MIT License, Phantasm is
licensed under the GNU General Public License v3.0. By contributing to the
project, you agree to license your contributions under the applicable license.
For more information, please read our licenses:

- [MIT License](clients/LICENSE)
- [GNU General Public License v3.0](LICENSE)

## Disclaimer

Phantasm is still in its early stages of development. Therefore, use it at your
own risk. With that being said, we are actively working on improving the
toolkits and adding essential features to make it more robust and secure. We
will be happy if you actually use it and provide us with feedback to make it
better.
