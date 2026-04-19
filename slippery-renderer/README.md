# Slippery-Renderer
This crate is the first implementation for a renderer of the buttery-engine game-engine project. Since I am very much still a beginner, this renderer's performance might slip sometimes. This is where it got it's name from.

## Resources used to create this implementation
- The basic implementation of the renderer has been taken from [here](https://sotrh.github.io/learn-wgpu/).
    - Instancing has been skipped as it was not necessary so far.
    - The code has been split between multiple files for easier extendability.
- The depth buffer implementation was majorly supported by copilot.
- The shadow calculation and final shader has been written by me.

## Things of note
Before adding the necessary abstraction layers, this renderer was able to target the web. It has since been broken. I've identified that it breaks if I enable the on_init in windowing.rs. No further analysis has taken place.