# todoist-cli
A rusty CLI for Todoist

## Motivation
I have been looking around for a todo style application and got used to using Todoist's web UI. The next logical step was to take that to the terminal, but instead of using some of the existing options, [for example](https:://github.com/sachaos/todoist), I took the chance to practice some Rust.

## Current status
Right now `todoist-cli` is in a very early stage: the CLI only supports `get`ing tasks and projects, and printing them.

## Planned work
A lot! I intend to support at least everything the [sync API offers](https://developer.todoist.com/sync/v8/#) offers. The first things I will be working on are:
* Table-like output to replace the standard DEBUG display format
* Getting the rest of the items
* Adding all the items
