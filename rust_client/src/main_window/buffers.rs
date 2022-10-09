pub struct GeometryBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub index_count: u32,
}

// bytemuck traits say that this can be converted to bytes and can be used
// with std::mem::zeroed()
// #[repr(C)] is for compatibility with shaders
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub const INDICES: &[u16] = &[0, 1, 2];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawMatrix {
    // We want to be able to pass a 4x4 matrix to a buffer on the gpu, and we use
    // bytemuck::cast_slice to enable that, but bytemuck::cast_slice doesn't operate
    // on cgmath::Matrix4, so this is a convenience struct so we can do
    // bytemuck::cast_slice(&[RawMatrix::new(cgmath_matrix)])
    matrix: [[f32; 4]; 4],
}

impl RawMatrix {
    pub fn new(matrix: cgmath::Matrix4<f32>) -> Self {
        Self {
            matrix: matrix.into(),
        }
    }
}
