// Vertex shader bindings

struct VertexOutput {
    [[location(0)]] tex_coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coord = fma(position, vec2<f32>(0.5, -0.5), vec2<f32>(0.5, 0.5));
    out.position = vec4<f32>(position, 0.0, 1.0);
    return out;
}

// Fragment shader bindings

[[group(0), binding(0)]] var r_tex_color: texture_2d<f32>;
[[group(0), binding(1)]] var r_tex_sampler: sampler;

struct Locals {
    transform: mat4x4<f32>;
};
[[group(0), binding(2)]] var<uniform> r_locals: Locals;
// [[group(0), binding(3)]] var<uniform> r_size: vec2<f32>;


fn get_checkerboard(size: vec2<i32>, pos: vec2<f32>) -> vec4<f32> {
    let pos_i32 = vec2<i32>(vec2<f32>(size) * pos);
    let grid_size = 10;
    if ((pos_i32.x / grid_size) % 2 == (pos_i32.y / grid_size) % 2) {
        return vec4<f32>(0.6, 0.6, 0.6, 1.0);
    } else {
        return vec4<f32>(0.4, 0.4, 0.4, 1.0);
    };
}

[[stage(fragment)]]
fn fs_main([[location(0)]] pos: vec2<f32>) -> [[location(0)]] vec4<f32> {
    let size = textureDimensions(r_tex_color);
    let bg = get_checkerboard(size, pos);

    let tex_coord = (r_locals.transform * vec4<f32>(pos, 0.0, 1.0)).xy;
    let sample = textureSample(r_tex_color, r_tex_sampler, tex_coord);

    if (tex_coord.x < 0.0 || tex_coord.x > 1.0 || tex_coord.y < 0.0 || tex_coord.y > 1.0) {
        // return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        return bg;
    } else {
        return sample;
    }
}
