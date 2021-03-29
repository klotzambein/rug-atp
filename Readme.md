# Agent Technology Practical - Final Project
By Andrei Voinea, Ivaylo Zhelev and Robin Kock

## Compiling
This project needs a recent version of the stable rust compiler (tested on
1.50.0). It is tested on linux, but should work on windows. Since we depend on
Imgui a c++ compiler might be required, If anything is missing it will most
likely give an error during the compilation.

To compile and run: `cargo run`.


## World representation
Each agent occupies exactly one tile. There is a bi-directional mapping from
agent to tile. Both agents and tiles are stored in one continous vector each.
Every step each agent specifies the action it would like to do. Then all the
confilcts are resolved and finally all the actions are applied and the next step
is started.

## Media used
[Random pixel characters](https://opengameart.org/content/random-pixel-characters) by [icedman](https://opengameart.org/users/icedman)