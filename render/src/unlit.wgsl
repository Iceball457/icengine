@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec3<f32>,
}

struct ModelMatrix {
    @location(5) model_mat_a: vec4<f32>,
    @location(6) model_mat_b: vec4<f32>,
    @location(7) model_mat_c: vec4<f32>,
    @location(8) model_mat_d: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec3<f32>,
}

fn construct_matrix(matrix: ModelMatrix) -> mat4x4<f32>{
    return mat4x4<f32>(
        matrix.model_mat_a,
        matrix.model_mat_b,
        matrix.model_mat_c,
        matrix.model_mat_d,
    );
}

@vertex
fn vs_unlit(
    vertex: VertexInput,
    model_matrix: ModelMatrix,
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = camera * construct_matrix(model_matrix) * vec4<f32>(vertex.position, 1.0);
    output.uv = vertex.uv;
    output.color = vertex.color;
    return output;
}

@fragment
fn fs_unlit(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
