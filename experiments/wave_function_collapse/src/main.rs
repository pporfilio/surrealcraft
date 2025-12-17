use image;

use std::sync::Arc; // Atomic ref counting

use wgpu::util::DeviceExt; // To use wgpu::Device::create_buffer_init

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use cgmath::Rotation3;

use wave_function_collapse::{INDICES, Instance, InstanceRaw, VERTICES, Vertex};

mod alg;
mod camera;
mod texture;

// This will store the state of our game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    demo_state: alg::DemoState,
    diffuse_texture: texture::Texture,
    texture_array: texture::TextureArray,
    camera: camera::OrthoCamera2D,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

// Rendering based on https://sotrh.github.io/learn-wgpu/beginner/

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = Self::get_gpu_instance();

        let surface = instance.create_surface(window.clone()).unwrap();

        // Couldn't figure out how to factor this out:
        // * issue with surface lifetime
        // * unsure how to set return type for a future, an await'd future, or an await?'d future.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        // Presume same issues as with the adapter
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_config = Self::get_surface_config(&surface, &adapter, size);

        // Abbreviation macro for
        // let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //    label: Some("Shader"),
        //    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        // });
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let imgx = 800;
        let imgy = 800;
        let initial_color = image::Rgba([255, 255, 255, 255]);
        let diffuse_rgba = image::RgbaImage::from_pixel(imgx, imgy, initial_color);
        let demo_state = alg::DemoState::new(diffuse_rgba, 100);

        // The compilier suggested this clone (acknowledging the performance impact).
        // The issue seemed to be that ImageRgba8 moves the value but then we tried
        // to use diffuse_rgba later to return it as part of the State struct which
        // wasn't allowed.
        let diffuse_dynamic = image::DynamicImage::ImageRgba8(demo_state.img.clone());

        let diffuse_texture =
            texture::Texture::new(&device, &queue, diffuse_dynamic, Some("test_image"));

        let diffuse_2_rgba =
            image::RgbaImage::from_pixel(imgx, imgy, image::Rgba([0, 255, 0, 255]));
        let diffuse_2_dynamic = image::DynamicImage::ImageRgba8(diffuse_2_rgba.clone());

        let mut texture_vec = Vec::<image::DynamicImage>::new();
        let diffuse_1_dynamic_for_array = image::DynamicImage::ImageRgba8(demo_state.img.clone());
        texture_vec.push(diffuse_1_dynamic_for_array);
        texture_vec.push(diffuse_2_dynamic);
        let texture_array =
            texture::TextureArray::new(&device, &queue, texture_vec, Some("texture_array"));

        // Make sure that if you add new instances to the Vec, you recreate the
        // instance_buffer as well as camera_bind_group. Otherwise, your new instances        // won't show up correctly.
        let mut instances = Vec::<Instance>::new();
        instances.push(Instance {
            scale: cgmath::Vector2 { x: 1.0, y: 1.0 },
            position: cgmath::Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
            texture_index: 0,
        });

        instances.push(Instance {
            scale: cgmath::Vector2 { x: 1.0, y: 1.0 },
            position: cgmath::Vector3 {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
            texture_index: 1,
        });

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES), // casts to &[u8]
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let camera = camera::OrthoCamera2D::new(size.width, size.height, 100, 0.025);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.set_view_proj(camera.build_view_projection_matrix());

        let camera_buffer = camera_uniform.get_camera_buffer(&device);

        let camera_bind_group_layout = camera_uniform.get_camera_bind_group_layout(&device);

        let camera_bind_group = camera_uniform.get_camera_bind_group(
            &device,
            &camera_buffer,
            &camera_bind_group_layout,
        );

        let render_pipeline = Self::get_render_pipeline(
            &device,
            &diffuse_texture.bind_group_layout,
            &camera_bind_group_layout,
            &texture_array.bind_group_layout,
            &shader,
            &surface_config,
        );

        Ok(Self {
            surface,
            device,
            queue,
            surface_config,
            is_surface_configured: false,
            window,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            demo_state,
            diffuse_texture,
            texture_array,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
        })
    }

    pub fn get_gpu_instance() -> wgpu::Instance {
        return wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
    }

    pub fn get_surface_config(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    pub fn get_render_pipeline(
        device: &wgpu::Device,
        // TODO: Probably want to pass this in as an array
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_array_bind_group_layout: &wgpu::BindGroupLayout,
        shader: &wgpu::ShaderModule,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &texture_array_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // A function marked with @vertex
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                // Wrapped in `Some` because technically optional.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // How to interpret our vertices
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            self.is_surface_configured = true;
            self.camera.resize(width, height);
        }
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Space, true) => self.step_algorithm(),
            _ => {
                self.camera.handle_key(code, is_pressed);
            }
        }
    }

    fn step_algorithm(&mut self) {
        alg::step_demo_image(&mut self.demo_state);
        self.diffuse_texture.update(
            &self.queue,
            image::DynamicImage::from(self.demo_state.img.clone()),
        );
    }

    pub fn update(&mut self) {
        self.camera.update_camera();
        self.camera_uniform
            .set_view_proj(self.camera.build_view_projection_matrix());
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // begin_render_pass() borrows encoder mutably (aka &mut self).
        // We can't call encoder.finish() until we release that mutable borrow.
        // The block tells Rust to drop any variables within it when the code leaves that scope.
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_texture.bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.texture_array.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        // Unsure if better to request at the end. Rendering might
        // be blocking the event loop anyway.
        self.window.request_redraw();

        Ok(())
    }
}

pub struct App {
    #[allow(unused)]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(event_loop: &EventLoop<State>) -> Self {
        let proxy = Some(event_loop.create_proxy());
        Self { state: None, proxy }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Return if we've already initialized
        if self.state.is_some() {
            println!("Resumed called when state already exists.");
            return;
        }

        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // TODO: Why are we doing this unconditionally in `resumed`, which can be
        // called multiple times?
        // Early in the tutorial it seemed fine because the only state in State
        // was `window`, but then they added more stuff to State...
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
                // Didn't seem to help so much with graphics being squished for a frame before resizing...
                state.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app)?;

    Ok(())
}

fn main() {
    let imgx = 800;
    let imgy = 800;

    let initial_color = image::Rgba([255, 255, 255, 255]);
    let mut imgbuf = image::RgbaImage::from_pixel(imgx, imgy, initial_color);

    let step = 100;
    let x_steps = imgx / step;
    let y_steps = imgy / step;
    println!("x_steps {} y_steps {}", x_steps, y_steps);
    for x in 0..x_steps {
        for y in 0..y_steps {
            let subimage = image::RgbaImage::from_pixel(
                imgx,
                imgy,
                image::Rgba([255 / (x + 1) as u8, 255 / (y + 1) as u8, 0, 255]),
            );
            image::imageops::replace(&mut imgbuf, &subimage, (x * step) as i64, (y * step) as i64);
        }
    }

    imgbuf.save("result.png").unwrap();

    let _ = run();
}
