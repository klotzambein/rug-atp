# Agent Technology Practical - Final Project
By Andrei Voinea, Ivaylo Zhelev and Robin Kock

## Compiling
This project needs a recent version of the stable rust compiler (tested on
1.50.0). It is tested on linux, but should work on windows. Since we depend on
Imgui a c++ compiler might be required, If anything is missing it will most
likely give an error during the compilation.

To compile: `cargo build --release`.

## Running
It is advised to compile and run the code in release mode, to accelerate the
initialization and stepping. The code can be run in two modes, interactive mode
and batch mode. In batch mode a path to a folder is given and all the contained
configs will be loaded run simultaneously in the background. The statistics will
be exported into the other given folder. To get more help run 
`cargo run --release -- batch --help`.

In interactive mode a simulation can be viewed as it progresses. Optionally a
config can also be specified. To get more help run 
`cargo run --release -- interactive --help`.

Finally for more options run
`cargo run --release -- --help`.

For our simulation we ran
`cargo run --release -- batch configs out`.

## World representation
Each agent occupies exactly one tile. There is a bi-directional mapping from
agent to tile. Both agents and tiles are stored in one continous vector each.
Every step each agent specifies the action it would like to do. Then all the
confilcts are resolved and finally all the actions are applied and the next step
is started.

## Media used
[Random pixel characters](https://opengameart.org/content/random-pixel-characters) by [icedman](https://opengameart.org/users/icedman)