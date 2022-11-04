use std::f32::consts::PI;

pub struct Camera {
    position: cgmath::Vector3<f32>,
    pitch_rad: f32,
    yaw_rad: f32,
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
    (in_deg * 2.0 * PI) / 360.0
}

pub fn rad_to_deg(in_rad: f32) -> f32 {
    in_rad * 360.0 / (2.0 * PI)
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
            pitch_min: -1.0 * (PI / 2.0) + 0.4,
            pitch_max: (PI / 2.0) - 0.4,
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
        // Setting yaw = 0 and pitch = 0, this look vector is (1, 0, 0)
        // In combination with the current implementation of build_view_projection_matrix,
        // when the camera is at the origin, this looks toward positive X as expected.

        cgmath::Vector3::new(
            f32::cos(self.yaw_rad()) * f32::cos(self.pitch_rad()),
            -1.0 * (f32::sin(self.yaw_rad()) * f32::cos(self.pitch_rad())),
            f32::sin(self.pitch_rad()),
        )
    }

    pub fn up_vector(&self) -> cgmath::Vector3<f32> {
        cgmath::Vector3::new(0.0, 0.0, 1.0)
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let pos = self.position();
        let lv = self.look_vector();
        let uv = self.up_vector();

        // Something funky is going on here where when the camera is looking toward
        // positive x: geometry is rendered correctly in screen space (+x is forward,
        // +y is to the left, +z is up) but when I rotate or move the camera to look at
        // +y geometry, the look vector is (0, 0, -1) and when I rotate the camera to
        // look at +z geometry, the look vector is (0, 1, 0).
        // So I _think_ look_to_rh interprets the vectors I pass to it differently than
        // I do. Since the geometry looked correct, I didn't want to change what I passed
        // to look_to_rh. So instead I changed how the look and up vector are calculated
        // to give the results I expect in geometry-space, and undo those changes here to
        // get whatever look_to_rh expects.
        // I think this is effectively just another linear transformation that could be
        // baked in to OPENGL_TO_WGPU_MATRIX, but I'm tired and I don't understand
        // the coordinate systems wgpu and cgmath use well enough.
        // I think this encapsulates all the coordinate transforms in build_view_projection_matrix
        // and external to the camera if I compare the camera look or position to my geometry
        // mesh representation it will work correctly.
        let view = cgmath::Matrix4::look_to_rh(
            cgmath::Point3::new(pos.x, pos.z, -pos.y),
            cgmath::Vector3::new(lv.x, lv.z, -lv.y),
            cgmath::Vector3::new(uv.x, uv.z, -uv.y),
        );
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_eps_f32(a: f32, b: f32, eps: f32) {
        assert!((a - b).abs() < eps);
    }

    #[test]
    fn set_pitch_rad() {
        let mut c = Camera::new(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
            640.0 / 480.0,
            45.0,
            0.1,
            100.0,
        );

        // Ensure that values greater than pitch_max are set to pitch_max, less than
        // pitch_min are set to pitch_min, and inbetween are not affected.
        c.set_pitch_rad(c.pitch_max + 2.0);
        assert_eq!(c.pitch_rad(), c.pitch_max);

        c.set_pitch_rad(c.pitch_min - 2.0);
        assert_eq!(c.pitch_rad(), c.pitch_min);

        let allowed_pitch = (c.pitch_max + c.pitch_min) / 2.0;
        c.set_pitch_rad(allowed_pitch);
        assert_eq!(c.pitch_rad(), allowed_pitch);
    }

    #[test]
    fn set_yaw_rad() {
        let mut c = Camera::new(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            0.0,
            0.0,
            640.0 / 480.0,
            45.0,
            0.1,
            100.0,
        );

        // Ensure that yaw values wrap from 2pi to 0 and from 0 to 2pi
        c.set_yaw_rad(2.0 * PI + 2.0);
        assert_eq_eps_f32(c.yaw_rad(), 2.0, 0.00001);

        c.set_yaw_rad(-2.0);
        assert_eq!(c.yaw_rad(), 2.0 * PI - 2.0);

        c.set_yaw_rad(PI);
        assert_eq!(c.yaw_rad(), PI);
    }
}
