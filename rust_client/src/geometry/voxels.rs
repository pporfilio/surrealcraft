use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    pub value: i32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Voxel {
    pub fn new(value: i32, r: f32, g: f32, b: f32) -> Self {
        Self { value, r, g, b }
    }
}

pub struct VoxelData<T: Copy> {
    data: Vec<T>,
    dimensions: cgmath::Vector3<u16>,
    voxel_count: u64,
}

fn read_i32(file: &mut File) -> Option<i32> {
    let mut buf: [u8; 4] = [0; 4];

    let read_result = file.read(&mut buf);
    let _ = match read_result {
        Ok(read_count) => read_count,
        Err(error) => {
            println!("Error reading i32 from file: {:?}", error);
            return None;
        }
    };
    Some(i32::from_be_bytes(buf))
}

impl<T: Copy + std::fmt::Debug> VoxelData<T> {
    pub fn new(dimensions: cgmath::Vector3<u16>, initial_value: T) -> Option<Self> {
        // Out of memory handling is complicated and not worth it at this point
        // https://www.crowdstrike.com/blog/dealing-with-out-of-memory-conditions-in-rust/

        let voxel_count = dimensions.x as u64 * dimensions.y as u64 * dimensions.z as u64;
        println!("Creating VoxelData with {:?} voxels", voxel_count);

        if voxel_count > usize::MAX as u64 || voxel_count > 2 * 1000 * 1000 * 1000 {
            return None;
        }

        let mut data: Vec<T> = Vec::with_capacity(voxel_count as usize);
        data.resize(voxel_count as usize, initial_value);

        Some(Self {
            data,
            // Conversion to usize should be safe because we checked against usize::MAX above
            dimensions: dimensions,
            voxel_count,
        })
    }

    fn array_offset(&self, indices: cgmath::Vector3<u16>) -> usize {
        // TODO: There's got to be a better way to do these calculations and conversions safely
        let offset: u64 = self.dimensions.x as u64 * self.dimensions.y as u64 * indices.z as u64
            + self.dimensions.x as u64 * indices.y as u64
            + indices.x as u64;
        if offset >= self.voxel_count {
            println!(
                "array_offset used to access voxel element {:?} but array has {:?} elements",
                offset, self.voxel_count
            );
        }
        if offset > usize::MAX as u64 {
            println!(
                "array_offset {:?} greater than usize::MAX of {:?}",
                offset,
                usize::MAX
            );
        }
        offset as usize
    }

    pub fn dimensions(&self) -> cgmath::Vector3<u16> {
        self.dimensions
    }

    pub fn set_data_at(&mut self, indices: cgmath::Vector3<u16>, value: T) {
        let offset = self.array_offset(indices);
        self.data[offset] = value;
    }

    pub fn data_at(&self, indices: cgmath::Vector3<u16>) -> T {
        let offset = self.array_offset(indices);
        let tmp = self.data[offset];
        tmp
    }
}

pub fn voxel_test_geometry() -> VoxelData<Voxel> {
    let mut vd = VoxelData::new(
        cgmath::Vector3 { x: 2, y: 2, z: 2 },
        Voxel::new(1, 0.8, 0.8, 0.4),
    )
    .unwrap();

    for x in 0..2 {
        for y in 0..2 {
            for z in 0..2 {
                vd.set_data_at(
                    cgmath::Vector3::new(x, y, z),
                    Voxel::new(1, x as f32 * 0.5, y as f32 * 0.5, z as f32 * 0.5),
                );
            }
        }
    }

    vd
}

pub fn voxel_data_from_file(path: &str) -> Option<VoxelData<Voxel>> {
    let file_open_result = File::open(path);
    let mut file = match file_open_result {
        Ok(file) => file,
        Err(error) => {
            println!("Error reading file {:?}: {:?}", path, error);
            return None;
        }
    };
    let mut buf: Vec<u8> = Vec::with_capacity(4);
    buf.resize(4, 0);

    let mut dimensions: cgmath::Vector3<u16> = cgmath::Vector3::new(0, 0, 0);

    for i in 0..3 {
        let tmp = read_i32(&mut file)?;
        if tmp > u16::MAX as i32 {
            println!(
                "Got dimension {:?} of {:?}, which is bigger than the max {:?}",
                i,
                tmp,
                u16::MAX
            );
        }
        dimensions[i] = tmp as u16;
    }

    let bytes_to_read = 16 * dimensions.x as usize * dimensions.y as usize * dimensions.z as usize;
    println!(
        "({:?}, {:?}, {:?})",
        dimensions.x, dimensions.y, dimensions.z
    );
    let mut voxel_data_buffer: Vec<u8> = Vec::with_capacity(bytes_to_read);
    voxel_data_buffer.resize(bytes_to_read, 0);

    let read_result = file.read(&mut voxel_data_buffer);
    let _ = match read_result {
        Ok(read_count) => read_count,
        Err(error) => {
            println!("Error reading voxel data file: {:?}", error);
            return None;
        }
    };

    let mut vd = VoxelData::new(
        dimensions,
        Voxel {
            value: 0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
        },
    )?;

    let mut count = 0;
    let mut buf: [u8; 4] = [0; 4];
    for z in 0..dimensions.z {
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
                buf.copy_from_slice(&voxel_data_buffer[count..count + 4]);
                let value = i32::from_be_bytes(buf);
                buf.copy_from_slice(&voxel_data_buffer[count + 4..count + 8]);
                let r = f32::from_be_bytes(buf);
                buf.copy_from_slice(&voxel_data_buffer[count + 8..count + 12]);
                let g = f32::from_be_bytes(buf);
                buf.copy_from_slice(&voxel_data_buffer[count + 12..count + 16]);
                let b = f32::from_be_bytes(buf);
                vd.set_data_at(cgmath::Vector3::new(x, y, z), Voxel::new(value, r, g, b));
                count += 16;
            }
        }
    }

    Some(vd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_size() {
        let vd: VoxelData<i32> = VoxelData::new(cgmath::Vector3::new(0, 0, 0), 0).unwrap();
        assert_eq!(vd.dimensions, cgmath::Vector3::new(0, 0, 0));
    }

    #[test]
    fn dimensions() {
        let vd: VoxelData<i32> = VoxelData::new(cgmath::Vector3::new(3, 4, 5), 0).unwrap();
        assert_eq!(vd.dimensions, cgmath::Vector3::new(3, 4, 5));
    }

    #[test]
    fn element_access() {
        let mut vd: VoxelData<i32> = VoxelData::new(cgmath::Vector3::new(3, 4, 5), 0).unwrap();
        for x in 0..3 {
            for y in 0..4 {
                for z in 0..5 {
                    vd.set_data_at(cgmath::Vector3::new(x, y, z), (x * y * z) as i32);
                }
            }
        }

        for x in 0..3 {
            for y in 0..4 {
                for z in 0..5 {
                    assert_eq!(
                        vd.data_at(cgmath::Vector3::new(x, y, z)),
                        (x * y * z) as i32
                    );
                }
            }
        }
    }

    #[test]
    fn read_voxel_data() -> std::io::Result<()> {
        // Have to handle errors from file operations. This is why the function
        // returns std::io::Result<()>, returns Ok(()) when everything goes well,
        // and has a ? after every file operation.
        // Also requires including use std::io::prelude::*;
        {
            let mut file = File::create("test.vd").unwrap_or_else(|error| {
                panic!("ARG");
            });
            let x: i32 = 3;
            let y: i32 = 4;
            let z: i32 = 5;
            file.write(&x.to_be_bytes())?;
            file.write(&y.to_be_bytes())?;
            file.write(&z.to_be_bytes())?;
            for z in 0..5 {
                for y in 0..4 {
                    for x in 0..3 {
                        file.write(&((x * y * z) as i32).to_be_bytes())?;
                        file.write(&(x as f32 / 3.0).to_be_bytes())?;
                        file.write(&(y as f32 / 4.0).to_be_bytes())?;
                        file.write(&(z as f32 / 5.0).to_be_bytes())?;
                    }
                }
            }
        }

        let vd: VoxelData<Voxel> = voxel_data_from_file("test.vd").unwrap_or_else(|| {
            panic!("Error reading file!");
        });

        for x in 0..3 {
            for y in 0..4 {
                for z in 0..5 {
                    let v = vd.data_at(cgmath::Vector3::new(x, y, z));
                    assert_eq!(v.value, x as i32 * y as i32 * z as i32);
                    assert_eq!(v.r, x as f32 / 3.0);
                    assert_eq!(v.g, y as f32 / 4.0);
                    assert_eq!(v.b, z as f32 / 5.0);
                }
            }
        }

        Ok(())
    }
}
