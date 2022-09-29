import numpy as np
import math


profile_1 = [
    (10, 1),
    (11, 2),
    (12, 3),
    (11, 4),
    (10, 5),
    (11, 6),
    (12, 7),
    (11, 8),
    (10, 9),
]
profile_2 = [(8, 1), (9, 2), (8, 3), (9, 4), (8, 5), (9, 6), (8, 7), (9, 8), (8, 9)]


def lerp(input_min, input_max, input_value, output_min, output_max):
    return output_min + (output_max - output_min) * (input_value - input_min) / (
        input_max - input_min
    )


def write_obj(vertices, faces):
    with open("vase.obj", "w") as f:
        for vertex in vertices:
            f.write(f"v {vertex[0]} {vertex[1]} {vertex[2]}\n")
        for face in faces:
            f.write(f"f {face[0]} {face[1]} {face[2]}\n")


if __name__ == "__main__":
    layers = []
    half_layer_length = 20
    for profile_point_1, profile_point_2 in zip(profile_1, profile_2):
        current_layer = []
        for th in np.linspace(0, math.pi, half_layer_length):
            scale = lerp(0, math.pi, th, profile_point_1[0], profile_point_2[0])
            current_layer.append(
                (math.cos(th) * scale, math.sin(th) * scale, profile_point_1[1])
            )
        for th in np.linspace(math.pi, 2 * math.pi, half_layer_length):
            scale = lerp(
                math.pi, 2 * math.pi, th, profile_point_2[0], profile_point_1[0]
            )
            current_layer.append(
                (math.cos(th) * scale, math.sin(th) * scale, profile_point_1[1])
            )
        layers.append(current_layer)

    vertices = [vertex for layer in layers for vertex in layer]
    faces = []
    layer_length = 2 * half_layer_length
    for layer_index in range(len(layers) - 1):
        # objs are 1-indexed?
        for vertex_index in range(1, layer_length):
            faces.append(
                (
                    layer_index * layer_length + vertex_index,
                    (layer_index + 1) * layer_length + vertex_index,
                    (layer_index * layer_length) + vertex_index + 1,
                )
            )
            faces.append(
                (
                    layer_index * layer_length + vertex_index + 1,
                    (layer_index + 1) * layer_length + vertex_index,
                    (layer_index + 1) * layer_length + vertex_index + 1,
                )
            )

    write_obj(vertices, faces)
