use cgmath::Rotation3;
use wgpu::util::DeviceExt;

pub struct GeometryBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub index_count: u32,
    pub instance_data: Vec<InstanceRaw>,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

// TODO
// RenderEntity and Entity are both terrible names because they don't include, e.g.,
// Camera and they include multiple instances.
// TODO: this probably shouldn't live in buffers, not sure where it should go
use super::super::geometry::geometry::TriangleMesh;
pub struct RenderEntity {
    pub mesh: TriangleMesh,
    pub instances: Vec<Instance>,
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
    pub fn get_vertex_buffer_layout_builder<'a>(
        shader_location_1: u32,
        shader_location_2: u32,
    ) -> VertexBufferLayoutBuilder<'a> {
        let builder = VertexBufferLayoutBuilder::new();
        let builder =
            builder.set_array_stride(std::mem::size_of::<Vertex>() as wgpu::BufferAddress);
        let builder = builder.set_step_mode(wgpu::VertexStepMode::Vertex);
        let builder = builder.set_attributes(vec![
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: shader_location_1,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: shader_location_2,
                format: wgpu::VertexFormat::Float32x3,
            },
        ]);
        builder
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

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }

    pub fn new() -> Self {
        Self {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
        }
    }
}

pub struct VertexBufferLayoutBuilder<'a> {
    layout: wgpu::VertexBufferLayout<'a>,
    attributes: Vec<wgpu::VertexAttribute>,
}

impl<'a> VertexBufferLayoutBuilder<'a> {
    pub fn new() -> Self {
        Self {
            attributes: vec![],
            layout: wgpu::VertexBufferLayout {
                attributes: &[],
                array_stride: 0 as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
            },
        }
    }

    pub fn set_array_stride(mut self, array_stride: wgpu::BufferAddress) -> Self {
        self.layout.array_stride = array_stride;
        self
    }

    pub fn set_step_mode(mut self, step_mode: wgpu::VertexStepMode) -> Self {
        self.layout.step_mode = step_mode;
        self
    }

    pub fn set_attributes(mut self, attributes: Vec<wgpu::VertexAttribute>) -> Self {
        self.attributes = attributes;
        self
    }

    pub fn build(&'a self) -> wgpu::VertexBufferLayout<'a> {
        let mut layout = self.layout.clone();
        layout.attributes = &self.attributes;
        layout
    }
}

impl InstanceRaw {
    // Why does the builder pattern work but just passing in parameters with static
    // lifetimes or lifetimes of 'a not work? I don't know, but I found the builder
    // pattern at https://github.com/gfx-rs/wgpu/discussions/2050
    pub fn get_vertex_buffer_layout_builder<'a>(
        shader_location_1: u32,
        shader_location_2: u32,
        shader_location_3: u32,
        shader_location_4: u32,
    ) -> VertexBufferLayoutBuilder<'a> {
        let builder = VertexBufferLayoutBuilder::new();
        let builder =
            builder.set_array_stride(std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress);
        let builder = builder.set_step_mode(wgpu::VertexStepMode::Instance);
        let builder = builder.set_attributes(vec![
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: shader_location_1,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: shader_location_2,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                shader_location: shader_location_3,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                shader_location: shader_location_4,
                format: wgpu::VertexFormat::Float32x4,
            },
        ]);
        builder
    }
}

pub fn geometry_buffers_from_mesh(
    device: &wgpu::Device,
    mesh: &TriangleMesh,
    instances: &Vec<Instance>,
) -> GeometryBuffers {
    let mut vertex_data: Vec<f32> = Vec::new();
    for (position, color) in mesh.vertices.iter().zip(mesh.colors.iter()) {
        vertex_data.push(position.x);
        vertex_data.push(position.y);
        vertex_data.push(position.z);
        vertex_data.push(color.x);
        vertex_data.push(color.y);
        vertex_data.push(color.z);
    }

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        // create_buffer_init needs plain u8 array. Bytemuck is a casting
        // library and we added some traits to struct Vertex to make it work
        // with bytemuck
        contents: bytemuck::cast_slice(&vertex_data[..]),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&mesh.indices[..]),
        usage: wgpu::BufferUsages::INDEX,
    });

    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    // TODO
    // GeometryBuffers should probably be refactored into geometry that has
    // a mesh, instance info, and a buffers sub-struct or something and a function
    // to update the buffers from updated mesh/instances and a way to indicate
    // what needs to be re-sent to the GPU.
    GeometryBuffers {
        vertex_buffer,
        index_buffer,
        vertex_count: mesh.vertices.len() as u32,
        index_count: mesh.indices.len() as u32,
        instance_data,
        instance_buffer,
        instance_count: instances.len() as u32,
    }
}
