struct GlobalUniforms {
    window_size: vec2<f32>;
};

var<private> f_color: vec4<f32>;
[[group(1), binding(0)]]
var t_total_radiance: texture_2d<f32>;
var<private> gl_FragCoord_1: vec4<f32>;
[[group(0), binding(0)]]
var<uniform> unnamed: GlobalUniforms;

fn into_srgbvf4_(linear: ptr<function, vec4<f32>>) -> vec4<f32> {
    let _e13 = (*linear);
    let _e15 = pow(_e13.xyz, vec3<f32>(0.4545454680919647, 0.4545454680919647, 0.4545454680919647));
    let _e17 = (*linear)[3u];
    return vec4<f32>(_e15.x, _e15.y, _e15.z, _e17);
}

fn main_1() {
    var param: vec4<f32>;

    let _e13 = gl_FragCoord_1;
    let _e16 = textureLoad(t_total_radiance, vec2<i32>(_e13.xy), 0);
    param = _e16;
    let _e17 = into_srgbvf4_((&param));
    f_color = _e17;
    return;
}

[[stage(fragment)]]
fn main([[builtin(position)]] gl_FragCoord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e3 = f_color;
    return _e3;
}
