## Summary

This repo is a collection of experiments and personal projects that are independent but work with each other somewhat. Check out the subdirectories for the various projects:

### [Perlin Experiment](experiments/perlin)

Playing around with a noise generation algorithm based on Perlin Noise.

![Terrain Output](experiments/perlin/perlin_2d_multi_terrain.png)

### [Rust Client](rust_client)

Contains a scene where you can move around and look at terrain made out of voxels, and a scene that demos ellipse-triangle collision detection and response in 3D.

The name is a bit aspirational -- it's an application that runs locally and doesn't have a backend or server component yet.

![Voxel Scene Screenshot](rust_client/images/voxel_scene.png)

### [Wave Function Collapse Experiment](experiments/wave_function_collapse)

Trying out a ["wave function collapse" (2D model synthesis)](https://en.wikipedia.org/wiki/Model_synthesis) algorithm on a 2D canvas implemented with wgpu. This was an opportunity to learn about working with textures in wgpu as well as experiment with an algorithm used in procedural generation.

![WFC Output](experiments/wave_function_collapse/wfc_terrain_2.png)

Copyright 2025 Parker Porfilio