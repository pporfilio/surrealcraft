// https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera

use cgmath;

use wgpu::util::DeviceExt; // To use wgpu::Device::create_buffer_init
use winit::keyboard::KeyCode;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    // I guess we don't store the buffer and bind group in the CameraUniform struct
    // so that CameraUniform can be bytemuck::Pod etc. and used directly as data for
    // the buffer contents.
    // Could of course add another layer here but I'm sticking closer to the tutorial for now.
    pub fn get_camera_buffer(self: &Self, device: &wgpu::Device) -> wgpu::Buffer {
        return device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[*self]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    }

    pub fn get_camera_bind_group_layout(
        self: &Self,
        device: &wgpu::Device,
    ) -> wgpu::BindGroupLayout {
        return device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
    }

    pub fn get_camera_bind_group(
        self: &Self,
        device: &wgpu::Device,
        camera_buffer: &wgpu::Buffer,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        return device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
    }

    pub fn set_view_proj(&mut self, view_proj_matrix: cgmath::Matrix4<f32>) {
        self.view_proj = view_proj_matrix.into();
    }
}

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool {
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

pub struct OrthoCamera2D {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub znear: f32,
    pub zfar: f32,
    pub speed: f32,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_zoom_in_pressed: bool,
    pub is_zoom_out_pressed: bool,
}

impl OrthoCamera2D {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32, speed: f32) -> Self {
        Self {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 0.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            left,
            right,
            top,
            bottom,
            znear: 0.1,
            zfar: 100.0,
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_zoom_in_pressed: false,
            is_zoom_out_pressed: false,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::ortho(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.znear,
            self.zfar,
        );

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn handle_key(&mut self, key: KeyCode, is_pressed: bool) -> bool {
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_up_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_down_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self) {
        if self.is_up_pressed {
            self.eye += cgmath::Vector3::new(0.0, self.speed, 0.0);
            self.target += cgmath::Vector3::new(0.0, self.speed, 0.0);
        }
        if self.is_down_pressed {
            self.eye -= cgmath::Vector3::new(0.0, self.speed, 0.0);
            self.target -= cgmath::Vector3::new(0.0, self.speed, 0.0);
        }
        if self.is_right_pressed {
            self.eye += cgmath::Vector3::new(self.speed, 0.0, 0.0);
            self.target += cgmath::Vector3::new(self.speed, 0.0, 0.0);
        }
        if self.is_left_pressed {
            self.eye -= cgmath::Vector3::new(self.speed, 0.0, 0.0);
            self.target -= cgmath::Vector3::new(self.speed, 0.0, 0.0);
        }
    }
}
