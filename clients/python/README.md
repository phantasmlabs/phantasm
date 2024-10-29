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

# Replace with your own function and parameters.
parameters = {...}
def schedule_meeting(...):
    pass

phantasm = Phantasm()
response = phantasm.get_approval(
    name="schedule_meeting",
    parameters=parameters
)

if response.approved:
    # Do this only if you trust the approvers.
    schedule_meeting(**response.parameters)
else:
    fallback()
```
