use super::geometry::TriangleMesh;
use std::fs::File;
use std::io::prelude::*;

pub fn read_obj(path: &str, color: cgmath::Vector3<f32>) -> Result<TriangleMesh, std::io::Error> {
    let file_open_result = File::open(path);
    let mut file = match file_open_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    // TODO: can I query the file's size to preallocate the buffer?
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    // TODO: can I use matching on string prefixes?
    // TODO: handle multiple objects
    // TODO: handle other parts of obj spec, like materials, texture coordinates, and normals
    let mut first_object = false;
    let mut tm = TriangleMesh::new(200, 200);
    for line in buf.lines() {
        if line.starts_with("o ") {
            continue;
        } else if line.starts_with("o ") {
            if !first_object {
                println!("Found object {:?}", line);
                first_object = true;
            } else {
                println!("Stopping at second object: {:?}", line);
                break;
            }
        } else if line.starts_with("v ") {
            let parts: Vec<&str> = line.split(" ").collect();
            let x: f32 = parts[1].parse().unwrap();
            let y: f32 = parts[2].parse().unwrap();
            let z: f32 = parts[3].parse().unwrap();
            tm.vertices.push(cgmath::Vector3::new(x, y, z));
            tm.colors.push(color);
        } else if line.starts_with("f") {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() > 4 {
                panic!("Faces with more than 3 vertices not supported in this obj reader.");
            }
            for i in 1..4 {
                let index_parts: Vec<&str> = parts[i].split("/").collect();
                tm.indices.push(index_parts[0].parse::<u32>().unwrap() - 1);
            }
        } else {
            println!("unhandled line: {:?}", line);
        }
    }
    tm.trim();
    Ok(tm)
}
