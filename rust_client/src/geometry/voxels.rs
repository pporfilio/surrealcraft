#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    value: i32,
    r: f32,
    g: f32,
    b: f32,
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

impl<T: Copy + std::fmt::Debug> VoxelData<T> {
    pub fn new(dimensions: cgmath::Vector3<u16>, initial_value: T) -> Option<Self> {
        // Out of memory handling is complicated and not worth it at this point
        // https://www.crowdstrike.com/blog/dealing-with-out-of-memory-conditions-in-rust/

        let voxel_count = dimensions.x as u64 * dimensions.y as u64 * dimensions.z as u64;
        println!("Createing VoxelData with {:?} voxels", voxel_count);

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
}
