# Dev Tools

For motivation and usage, see top-level README.md

### Developers

The stack is roughly:

* Tauri: GUI glue
* Dominator: UI framework

There are two ways of executing things, either via GUI or CLI.

Therefore, most of the heavy lifting is done in `shared-cli`, so that it can be executed from either place.

For right now, pretty much all the commands are just done by executing the root Taskfile - this is just a layer on top of that

But, eventually, this could change....