struct GlobalUniforms {
    window_size: vec2<f32>;
};

struct FragmentOutput {
    [[location(0)]] member: vec4<f32>;
    [[location(1)]] member_1: vec4<f32>;
    [[location(2)]] member_2: vec2<f32>;
};

var<private> f_albedo_lin: vec4<f32>;
var<private> o_albedo_lin_1: vec4<f32>;
var<private> f_radiance_lin: vec4<f32>;
var<private> o_radiance_lin_1: vec4<f32>;
var<private> f_normal: vec2<f32>;
var<private> o_normal_1: vec2<f32>;
[[group(0), binding(0)]]
var<uniform> unnamed: GlobalUniforms;

fn main_1() {
    let _e11 = o_albedo_lin_1;
    f_albedo_lin = _e11;
    let _e12 = o_radiance_lin_1;
    f_radiance_lin = _e12;
    let _e13 = o_normal_1;
    f_normal = _e13;
    return;
}

[[stage(fragment)]]
fn main([[location(0)]] o_albedo_lin: vec4<f32>, [[location(1)]] o_radiance_lin: vec4<f32>, [[location(2)]] o_normal: vec2<f32>) -> FragmentOutput {
    o_albedo_lin_1 = o_albedo_lin;
    o_radiance_lin_1 = o_radiance_lin;
    o_normal_1 = o_normal;
    main_1();
    let _e9 = f_albedo_lin;
    let _e10 = f_radiance_lin;
    let _e11 = f_normal;
    return FragmentOutput(_e9, _e10, _e11);
}
