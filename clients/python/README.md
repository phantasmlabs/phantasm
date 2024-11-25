[![GitHub](https://tinyurl.com/5xetszfe)][github]
[![Discord](https://tinyurl.com/5fv83u2h)][discord]
[![License](https://tinyurl.com/mrxn8fvz)][license]

[discord]: https://discord.gg/dgevsYhh7P
[github]: https://github.com/phantasmlabs/phantasm
[license]: https://github.com/phantasmlabs/phantasm/blob/main/LICENSE

![Phantasm](https://phantasm-assets.s3.amazonaws.com/banners/0.1.0.png)

This client library allows you to interact with Phantasm's receiver server from
your Python codebase. It provides a simple and easy-to-use interface complete
with type hints and documentation. Here's a quick example to get you started
adding an approval workflow to your function:

```py
from phantasmpy import Phantasm
phantasm = Phantasm()

# Replace with your own function.
@phantasm.require_approval()
def schedule_meeting(...):
    pass

schedule_meeting(...)
```

## Advanced Usage With LangChain

In real-world applications, you may want to use Phantasm with LangChain to
utilize the full power of LangChain's features. For the sake of simplicity, we
will be using OpenAI's GPT model as our language model. Here's how you can build
a tool-powered AI agent with human-in-the-loop capabilities:

Requirements:

```bash
pip install langchain langchain-openai phantasmpy
```

```py
import os
from langchain_core.tools import tool
from langchain_core.messages import AIMessage
from langchain_openai import ChatOpenAI
from phantasmpy import Phantasm

os.environ["OPENAI_API_KEY"] = "xxx"
llm = ChatOpenAI(model="gpt-4o-mini")

@tool
def publish_blog(topic: str):
    # Add code to draft and publish the blog post.
    print(f"A blog post ({topic}) is published.")

@tool
def send_email(to: str, subject: str, body: str):
    # Add code to send an email using an email API.
    print(f"An email is sent to {to}.")

tools = [publish_blog, send_email]
llm_with_tools = llm.bind_tools(tools)
phantasm = Phantasm()

def call_tools(message: AIMessage):
    tool_map = {tool.name: tool for tool in tools}
    for tool_call in message.tool_calls:
        name, args = tool_call["name"], tool_call["args"]
        response = phantasm.get_approval(name=name, parameters=args)

        if response.approved:
            tool_map[name].invoke(response.parameters)
        else:
            print(f"Tool call {name} is rejected.")

chain = llm_with_tools | call_tools
chain.invoke("Remind elon@x.com about the Monday meeting at 9AM")
```
