// A sample.
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

struct Uniforms {
    transform: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

let PI: f32 = 3.14159;
let LIMIT: i32 = 10;

fn area(radius: f32) -> f32 {
    return PI * radius * radius;
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) uv: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.transform * vec4<f32>(position, 1.0);
    out.uv = uv;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base = textureSample(tex, samp, in.uv);
    var total = 0.0;
    for (var i = 0; i < LIMIT; i = i + 1) {
        total = total + area(f32(i)) * sin(uniforms.time);
    }
    if (base.a < 0.5) {
        discard;
    }
    return vec4<f32>(base.rgb * total, 1.0);
}
