use cgmath::SquareMatrix;

//  UNIFORM BUFFER - A blob of data that is available to every invocation of a set of shaders. Used to store our view projection matrix.

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    //  Uniforms require 4 float (16 byte) spacing, we need to use padding fields
    pub _padding: u32,
    pub color: [f32; 3],
    pub _padding2: u32,
}

#[repr(C)]  //  needed for Rust to store the data for shaders correctly
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]    //  needed so we can store it in a buffer
pub struct CameraUniform {
    view_position: [f32; 4],
    //  cgmath cant be used directly with bytemuck, so we have to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &crate::game::camera::Camera, projection: &crate::game::camera::Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}