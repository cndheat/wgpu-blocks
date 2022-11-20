//  Vertex Shader

struct VertexOutput {   //  Struct to store the output of our shader
    //  tells wgpu this is the value we want to use as the vertex's clip coords
    @builtin(position) clip_position: vec4<f32>,
};

//  @vertex - shader program entry point and expects a u32 input 'in_vertex_input'
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    //  variables defined with 'var' can be modified but must specify their type
    var out: VertexOutput;
    //  casts in_vertex_index to i32, then casts the result to f32
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    //  variables defined with 'let' can have their type inferred but cannot be changed during the shader
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

//  Fragment Shader

//  frag shader entry point
@fragment
//  @location(0) tells WGPu to store the vec4 value returned by the func in the first color target
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}