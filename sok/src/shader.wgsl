//vertex shader

//declare variables we use 
struct VertexOutput {
    //clip coord
    @builtin(position) clip_position: vec4f,
    //0..n
    @location(0) position: vec2f,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    //in_vertex_index will be 0..n
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.position = vec2f(x, y);
    out.clip_position = vec4f(x, y, 0.5, 0.5);
    return out;
}
 
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(0.36, 0.60, 0.57, 1.0);
}