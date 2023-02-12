use super::buffers::GeometryBuffers;
use super::buffers::InstanceRaw;
use super::buffers::RawMatrix;
use super::buffers::Vertex;
use super::texture;
use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;
use winit::window::Window;

use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Matrix4UniformInfo {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group_number: u32,
}

impl Matrix4UniformInfo {
    pub fn new(
        device: &wgpu::Device,
        initial_value: cgmath::Matrix4<f32>,
        bind_group_number: u32,
        buffer_label: &str,
        bind_group_layout_label: &str,
        bind_group_label: &str,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(buffer_label),
            contents: bytemuck::cast_slice(&[RawMatrix::new(initial_value)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    // Indicates buffer will not change size (as it would for an array e.g.)
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some(bind_group_layout_label),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some(bind_group_label),
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
            bind_group_number,
        }
    }
}

pub struct WGPUState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub camera_uniform: Matrix4UniformInfo,
    pub depth_texture: texture::Texture,
}

// https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#first-some-housekeeping-state
impl WGPUState {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };

        // adapter is a handle to our graphics card.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // Options are HighPerformance and LowPower. Unclear which default() picks.
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // TODO: WASM
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Draw to screen
            format: surface.get_supported_formats(&adapter)[0],
            width: if size.width < 1 { 1 } else { size.width },
            height: if size.height < 1 { 1 } else { size.height },
            // Cap display rate at display's framerate. Effectively VSync.
            // wgpu::PresentMode::Mailbox will not block but screen updates
            // will still not tear.
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        // Can be shortened to
        // let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("..\\shaders\\shader.wgsl").into()),
        });

        let camera_uniform = Matrix4UniformInfo::new(
            &device,
            cgmath::Matrix4::identity(),
            0,
            "Camera Buffer",
            "Camera Bind Group Layout",
            "Camera Bind Group",
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_uniform.bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                // What vertex format we want to pass to the vertex shader.
                buffers: &[
                    Vertex::get_vertex_buffer_layout_builder(0, 1).build(),
                    InstanceRaw::get_vertex_buffer_layout_builder(5, 6, 7, 8).build(),
                ],
            },
            // Wrapped in Some b/c fragment is technically optional
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,                  // our surface's format
                    blend: Some(wgpu::BlendState::REPLACE), // new color data should replace old color data
                    write_mask: wgpu::ColorWrites::ALL,     // Write all red, green, blue, and alpha
                })],
            }),
            // How to interpret our vertices when converting them to triangles
            primitive: wgpu::PrimitiveState {
                // Every 3 vertices will correspond to one triangle
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // cull_mode: None,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            camera_uniform,
            depth_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn update(&mut self, view_proj_matrix: cgmath::Matrix4<f32>) {
        // Apparently the usual way to do this is to create a separate buffer known
        // as a "staging buffer" and then copy into the camera_uniform.buffer so that
        // camera_uniform.buffer is only accessible by the gpu.
        self.queue.write_buffer(
            &self.camera_uniform.buffer,
            0,
            bytemuck::cast_slice(&[RawMatrix::new(view_proj_matrix)]),
        );
    }

    pub fn render(&mut self, geometry: &Vec<GeometryBuffers>) -> Result<(), wgpu::SurfaceError> {
        //sleep(Duration::new(2, 0));

        // Get texture to render to
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Encoder is a command buffer that we use to send commands to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // Mutably borrows _render_pass until the end of this block.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(
                self.camera_uniform.bind_group_number,
                &self.camera_uniform.bind_group,
                &[],
            );
            for g in geometry {
                render_pass.set_vertex_buffer(0, g.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, g.instance_buffer.slice(..));
                render_pass.set_index_buffer(g.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                // Note in the tutorial when adding instancing that
                // Make sure if you add new instances to the Vec, that you recreate the
                // instance_buffer and as well as camera_bind_group, otherwise your new
                // instances won't show up correctly.
                //
                // This is where @builtin(vertex_index) comes from.
                render_pass.draw_indexed(0..g.index_count, 0, 0..g.instance_count as _);
            }
        }
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
