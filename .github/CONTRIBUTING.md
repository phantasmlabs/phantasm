# Contributing Guidelines

First off, thank you for considering to contribute to this project!

Phantasm uses a wide-range of technologies and tools from different software
development ecosystems. Therefore, we welcome contributions from all skill
levels and backgrounds. We believe that a diverse community leads to a better
product.

## Project Structure

Phantasm consists of three main components: Server, Dashboard, and Client. The
Dashboard and Client components are located in the _dashboard_ and _clients_
directories, respectively, while the Server component is located in the _src_
directory.

Before you get started with contributing, you might need to set up your local
development environment by installing the necessary dependencies. These are some
of the primary technologies used to build Phantasm by its components:

- **Server**: Rust, Tonic, gRPC, Tokio, Tungstenite
- **Dashboard**: TypeScript, SvelteKit, Tailwind CSS
- **Client**: Varies depending on the programming language

## Where to Start

If you are new to the project, we recommend you to start by reading the
project's documentation. The documentation provides an overview of the project,
its components, and how they work together. If you have any questions, feel free
to ask in the project's Discord server.

After familiarizing yourself with the project structure, you can start to work
on an issue that is listed in the Issues section of the project's repository.
Some great issues to start with are the ones labeled as **good first issue**.

## Style Guides

To maintain a consistent codebase, we follow a set of style guides for each
component of the project. These style guides include settings from **rustfmt**
for the Server, **Prettier** for the Dashboard, and the respective formatters
for the Client Libraries, that you can find in their configuration files, such
as **Black** for Python.

### Commit Messages & PRs

To ensure that the project's commit history is clean and easy to read, we follow
the Conventional Commits specification. When you are pushing changes to the
project or creating a pull request, please make sure to follow this convention.
In short, a commit message or a pull request title should look like this:

```
<type>(optional scope): <description>
```

Source: [Convention Commits](https://www.conventionalcommits.org)
