//  Vertex Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {   //  Struct to store the output of our shader
    //  tells wgpu this is the value we want to use as the vertex's clip coords
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

//  @vertex - shader program entry point and expects a u32 input 'in_vertex_input'
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    //  variables defined with 'var' can be modified but must specify their type
    var out: VertexOutput;
    //  casts in_vertex_index to i32, then casts the result to f32
    //  variables defined with 'let' can have their type inferred but cannot be changed during the shader
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

//  Fragment Shader

//  frag shader entry point
@fragment
//  @location(0) tells WGPu to store the vec4 value returned by the func in the first color target
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}