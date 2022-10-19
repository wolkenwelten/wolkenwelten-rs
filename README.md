# WolkenWelten
This is the rust port of the WolkenWelten codebase, everything is still super early so prepare for pretty much nothing
working whatsoever.

Also keep in mind that this is the first Rust I've ever written, so if you spot some ugly bits of code
be sure to open a PR so as to improve the quality of the codebase :)


# General direction
I'm currently porting over most of the nice bits and pieces from the C engine, after that is done and things have settled
down a bit I'll be looking into integrating v8, by either using deno_core or hooking into rusty_v8 directly.      


# Current ToDo
- [X] Chunk fade
- [X] "Infinite" world
- [X] Simple placeholder worldgen
- [X] Remove hidden surfaces from BlockMeshes
- [X] Simple player controls (gravity/collision with the world)
- [X] Chunk/BlockMesh GC
- [X] Unit Tests
- [ ] Frustum culling (port from WW)
- [ ] CI running tests/fmt/clippy
- [ ] Greedy meshing (port from WW)
- [ ] Lighting (port from WW, without ASM/SIMD (for now))
- [ ] Nicer player movement (port from WW)
