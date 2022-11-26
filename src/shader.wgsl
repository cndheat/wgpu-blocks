//  Vertex Shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)   //  specify which bind group we're using in the shader. the number is determined by our render_pipeline_layout.
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {  
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

//  @vertex - shader program entry point and expects a u32 input 'in_vertex_input'
@vertex

//  variables defined with 'var' can be modified but must specify their type
//  variables defined with 'let' can have their type inferred but cannot be changed during the shader
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);  //  Vector on the right and matrices go left in order of importance
    return out;
}

//  Fragment Shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

//  frag shader entry point
@fragment
//  @location(0) tells WGPu to store the vec4 value returned by the func in the first color target
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}