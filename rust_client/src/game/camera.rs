use std::f32::consts::PI;

pub struct Camera {
    position: cgmath::Vector3<f32>,
    pitch_rad: f32,
    yaw_rad: f32,

    // eye: cgmath::Point3<f32>,
    // target: cgmath::Point3<f32>,
    // up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,

    pitch_min: f32,
    pitch_max: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub fn deg_to_rad(in_deg: f32) -> f32 {
    (in_deg * 2.0 * PI) / 180.0
}

pub fn rad_to_deg(in_rad: f32) -> f32 {
    in_rad * 180.0 / (2.0 * PI)
}

pub fn fmod(num: f32, denom: f32) -> f32 {
    num - (denom * (f32::floor(num / denom)))
}

// The coordinate system in Wgpu is based on DirectX, and Metal's coordinate systems.
// That means that in normalized device coordinates (opens new window)the x axis and
// y axis are in the range of -1.0 to +1.0, and the z axis is 0.0 to +1.0. The cgmath
// crate (as well as most game math crates) is built for OpenGL's coordinate system.
impl Camera {
    pub fn new(
        position: cgmath::Vector3<f32>,
        pitch_rad: f32,
        yaw_rad: f32,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            position,
            pitch_rad,
            yaw_rad,
            aspect,
            fovy,
            znear,
            zfar,
            pitch_min: (PI / 2.0) + 0.2,
            pitch_max: (PI / 2.0) - 0.2,
        }
    }

    pub fn pitch_rad(&self) -> f32 {
        self.pitch_rad
    }

    pub fn set_pitch_rad(&mut self, pitch_rad: f32) {
        if self.pitch_rad == pitch_rad {
            return;
        }
        self.pitch_rad = pitch_rad;
        if self.pitch_rad < self.pitch_min {
            self.pitch_rad = self.pitch_min;
        }
        if self.pitch_rad > self.pitch_max {
            self.pitch_rad = self.pitch_max;
        }
    }

    pub fn set_pitch_deg(&mut self, pitch_deg: f32) {
        self.set_pitch_rad(deg_to_rad(pitch_deg));
    }

    pub fn add_pitch_deg(&mut self, pitch_deg_delta: f32) {
        self.set_pitch_rad(self.pitch_rad + deg_to_rad(pitch_deg_delta));
    }

    pub fn yaw_rad(&self) -> f32 {
        self.yaw_rad
    }

    pub fn set_yaw_rad(&mut self, yaw_rad: f32) {
        if self.yaw_rad == yaw_rad {
            return;
        }
        self.yaw_rad = yaw_rad;
        if self.yaw_rad < 0.0 {
            self.yaw_rad = 2.0 * PI + fmod(self.yaw_rad, -2.0 * PI);
        }
        if self.yaw_rad > 2.0 * PI {
            self.yaw_rad = fmod(self.yaw_rad, 2.0 * PI);
        }
    }

    pub fn set_yaw_deg(&mut self, yaw_deg: f32) {
        self.set_yaw_rad(deg_to_rad(yaw_deg));
    }

    pub fn add_yaw_deg(&mut self, yaw_deg_delta: f32) {
        self.set_yaw_rad(self.yaw_rad + deg_to_rad(yaw_deg_delta));
    }

    pub fn set_position(&mut self, position: cgmath::Vector3<f32>) {
        self.position = position;
    }

    pub fn add_position_delta(&mut self, position_delta: cgmath::Vector3<f32>) {
        self.position += position_delta;
    }

    pub fn position(&self) -> cgmath::Point3<f32> {
        cgmath::Point3::new(self.position.x, self.position.y, self.position.z)
    }

    pub fn look_vector(&self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(
            f32::cos(self.yaw_rad() * f32::cos(self.pitch_rad())),
            f32::sin(self.pitch_rad()),
            f32::sin(self.yaw_rad()) * f32::cos(self.pitch_rad()),
        )
    }

    pub fn up_vector(&self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view =
            cgmath::Matrix4::look_to_rh(self.position(), self.look_vector(), self.up_vector());
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
