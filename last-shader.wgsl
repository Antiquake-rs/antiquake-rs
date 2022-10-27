struct Globals {
    camera_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    light_view_proj: mat4x4<f32>,
    light_pos: vec4<f32>,
    light_color: vec4<f32>, // not used
};

@group(0) @binding(0) var<uniform> u_Globals: Globals;
//!include globals.inc

struct Debug {
    color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> c_Debug: Debug;

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec4 a_Color;

struct Varyings {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn main_vs(@location(0) pos: vec4<f32>, @location(1) color: vec4<f32>) -> Varyings {
    return Varyings(
        u_Globals.view_proj * pos,
        color,
    );
}

@fragment
fn main_fs(in: Varyings) -> @location(0) vec4<f32> {
    return in.color;
}
