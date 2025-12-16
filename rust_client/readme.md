This is an experimental project for 3D rendering and interaction. The default scene renders voxels and allows moving around/looking around the scene. To run, you need to generate the voxel data first since it's a larger file and I don't have git lfs set up.

cd ../terrain_generation
python3 voxeldata.py perlin_2d_multi_terrain.png --output-file perlin_terrain.vd --with-heightmap perlin_2d_multi_heightmap.png

Once the voxels are generated, run from this directory with `cargo run`.

Example screenshot when running the voxel scene:
![Voxel Scene Screenshot](images/voxel_scene.png)

