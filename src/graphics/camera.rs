use crate::prelude::*;

pub trait Camera {
    fn projection_matrix(&self) -> Mat4;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PerspectiveCamera {
    pub view_dir: Vec3,
    pub position: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub aspect_ratio: f32,
}

impl PerspectiveCamera {
    pub fn new(screen_size: Vec2, position: Vec3, view_dir: Vec3) -> Self {
        Self {
            view_dir,
            position,
            up: Vec3::Y,
            fov: 70f32.to_radians(),
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: screen_size.x / screen_size.y,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.view_dir, self.position, self.up);
        let projection =
            Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near_clip, self.far_clip);

        projection * view
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OrthographicCamera {
    pub view_dir: Vec3,
    pub position: Vec3,
    pub orthographic_size: Vec2,
    pub up: Vec3,
    pub near: f32,
    pub far: f32,
}

impl OrthographicCamera {
    pub fn new(orthographic_size: Vec2, position: Vec3, view_dir: Vec3) -> Self {
        Self {
            view_dir,
            position,
            orthographic_size,
            up: Vec3::Y,
            near: 0.1,
            far: 1000.0,
        }
    }

    pub fn new_2d(orthographic_size: Vec2, position: Vec2) -> Self {
        Self::new(orthographic_size, position.extend(-10.0), Vec3::NEG_Z)
    }
}

impl Camera for OrthographicCamera {
    fn projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.view_dir, self.position, self.up);
        let half_size = self.orthographic_size / 2.0;
        let projection = Mat4::orthographic_rh(
            -half_size.x,
            half_size.x,
            -half_size.y,
            half_size.y,
            self.near,
            self.far,
        );

        projection * view
    }
}

impl<'a> Context<'a> {
    pub fn set_projection_matrix(&mut self, mat: Mat4) {
        self.handle
            .queue
            .write_buffer(&self.handle.camera_buffer, 0, bytemuck::cast_slice(&[mat]));
    }

    pub fn set_camera(&mut self, camera: impl Camera) {
        self.set_projection_matrix(camera.projection_matrix())
    }

    pub fn camera_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.handle.camera_bind_group_layout
    }

    pub fn camera_bind_group(&self) -> &wgpu::BindGroup {
        &self.handle.camera_bind_group
    }

    pub fn screen_size(&self) -> Vec2 {
        let size = self.handle.window.inner_size();
        Vec2::new(size.width as f32, size.height as f32)
    }
}
