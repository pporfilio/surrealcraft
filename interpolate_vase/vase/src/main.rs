use std::fs::File;
use std::io::Write;

fn make_profiles() -> (Vec<(f32, f32)>, Vec<(f32, f32)>) {
    let v1 = vec![
        (10.0, 1.0),
        (11.0, 2.0),
        (12.0, 3.0),
        (11.0, 4.0),
        (10.0, 5.0),
        (11.0, 6.0),
        (12.0, 7.0),
        (11.0, 8.0),
        (10.0, 9.0),
    ];
    let v2 = vec![
        (8.0, 1.0),
        (9.0, 2.0),
        (8.0, 3.0),
        (9.0, 4.0),
        (8.0, 5.0),
        (9.0, 6.0),
        (8.0, 7.0),
        (9.0, 8.0),
        (8.0, 9.0),
    ];
    (v1, v2)
}

fn lerp(input_min: f32, input_max: f32, input_value: f32, output_min: f32, output_max: f32) -> f32 {
    output_min + (output_max - output_min) * (input_value - input_min) / (input_max - input_min)
}

fn linspace(start: f32, end: f32, n: u32) -> Vec<f32> {
    let step = (end - start) / (n - 1) as f32;
    let mut result: Vec<f32> = Vec::new();
    for i in 0..n - 1 {
        result.push(start + i as f32 * step);
    }
    result.push(end);
    println!("{:?}", result.len());
    result
}

fn write_obj(
    vertices: Vec<(f32, f32, f32)>,
    faces: Vec<(u32, u32, u32)>,
) -> Result<(), std::io::Error> {
    let mut file = File::create("vase.obj")?;
    for vertex in vertices {
        write!(file, "v {} {} {}\n", vertex.0, vertex.1, vertex.2)?;
    }
    for face in faces {
        write!(file, "f {} {} {}\n", face.0, face.1, face.2)?;
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let (profile_1, profile_2) = make_profiles();

    let mut layers: Vec<Vec<(f32, f32, f32)>> = Vec::new();
    let layer_length = 80;
    for (profile_point_1, profile_point_2) in profile_1.iter().zip(profile_2.iter()) {
        println!("{:?}, {:?}", profile_point_1, profile_point_2);
        let mut current_layer: Vec<(f32, f32, f32)> = Vec::new();
        for th in linspace(0.0, 2.0 * std::f32::consts::PI, layer_length).iter() {
            let scale = lerp(-1.0, 1.0, th.cos(), profile_point_1.0, profile_point_2.0);
            current_layer.push((th.cos() * scale, th.sin() * scale, profile_point_1.1));
        }
        layers.push(current_layer);
    }

    let mut faces: Vec<(u32, u32, u32)> = Vec::new();
    for layer_index in 1..layers.len() as u32 - 1 {
        for vertex_index in 1..layer_length {
            faces.push((
                layer_index * layer_length + vertex_index,
                (layer_index + 1) * layer_length + vertex_index,
                (layer_index * layer_length) + vertex_index + 1,
            ));
            faces.push((
                layer_index * layer_length + vertex_index + 1,
                (layer_index + 1) * layer_length + vertex_index,
                (layer_index + 1) * layer_length + vertex_index + 1,
            ))
        }
    }

    let mut vertices: Vec<(f32, f32, f32)> = Vec::new();
    for layer in layers {
        for vertex in layer {
            vertices.push(vertex);
        }
    }

    write_obj(vertices, faces)?;

    Ok(())
}
