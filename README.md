Entity Rust Benchmark
=============

The idea I had for this benchmark is to implement a simple simulation. An amount of agents will walk
through a 2d field with obstacles. They will walk in random directions and will use line of sight
to avoid obstacles.

Things we need:

  - A map, visible on the screen
  - A list of entities on that map visible on the map
  - Some system that moves the entities around
  - Some system that calculates the lines of sight
  - Optionial: visualize the lines of sight

And then we can figure out what the bottlenecks are.

So first:

  - [ ] Find out what data structures what we should use, probably by finding out
  how we are going to do the calculations, I suspect ncollide.
  - [ ]