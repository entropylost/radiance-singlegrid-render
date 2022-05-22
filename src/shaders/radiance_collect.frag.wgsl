struct RadianceCollectUniforms {
    light_directions: u32;
};

struct GlobalUniforms {
    window_size: vec2<f32>;
};

[[group(1), binding(0)]]
var<uniform> unnamed: RadianceCollectUniforms;
[[group(3), binding(0)]]
var t_directional_radiance: texture_2d_array<u32>;
var<private> gl_FragCoord_1: vec4<f32>;
[[group(2), binding(0)]]
var t_albedo: texture_2d<f32>;
var<private> f_total_radiance: vec4<f32>;
[[group(2), binding(1)]]
var t_radiance: texture_2d<f32>;
[[group(0), binding(0)]]
var<uniform> unnamed_1: GlobalUniforms;
[[group(2), binding(2)]]
var t_normal: texture_2d<f32>;

fn from_rgb9e5u1_(encoded: ptr<function, u32>) -> vec3<f32> {
    var exponent: i32;
    var scale: f32;
    var v: vec3<f32>;

    let _e27 = (*encoded);
    exponent = (bitcast<i32>((_e27 >> bitcast<u32>(27))) - 15);
    let _e32 = exponent;
    scale = exp2(f32((_e32 - 9)));
    let _e36 = (*encoded);
    let _e39 = (*encoded);
    let _e44 = (*encoded);
    v = vec3<f32>(f32((_e36 & 511u)), f32(((_e39 >> bitcast<u32>(9)) & 511u)), f32(((_e44 >> bitcast<u32>(18)) & 511u)));
    let _e50 = v;
    let _e51 = scale;
    return (_e50 * _e51);
}

fn main_1() {
    var radiance: vec3<f32>;
    var i: u32;
    var param: u32;
    var albedo: vec4<f32>;

    radiance = vec3<f32>(0.0, 0.0, 0.0);
    i = 0u;
    loop {
        let _e27 = i;
        let _e29 = unnamed.light_directions;
        if ((_e27 < _e29)) {
            let _e31 = gl_FragCoord_1;
            let _e33 = vec2<i32>(_e31.xy);
            let _e34 = i;
            let _e38 = vec3<i32>(_e33.x, _e33.y, bitcast<i32>(_e34));
            let _e44 = textureLoad(t_directional_radiance, vec2<i32>(_e38.x, _e38.y), i32(_e38.z), 0);
            param = _e44.x;
            let _e46 = from_rgb9e5u1_((&param));
            let _e47 = radiance;
            radiance = (_e47 + _e46);
            continue;
        } else {
            break;
        }
        continuing {
            let _e49 = i;
            i = (_e49 + bitcast<u32>(1));
        }
    }
    let _e52 = gl_FragCoord_1;
    let _e55 = textureLoad(t_albedo, vec2<i32>(_e52.xy), 0);
    albedo = _e55;
    let _e56 = radiance;
    let _e57 = albedo;
    let _e60 = gl_FragCoord_1;
    let _e63 = textureLoad(t_radiance, vec2<i32>(_e60.xy), 0);
    let _e65 = ((_e56 * _e57.xyz) + _e63.xyz);
    let _e67 = albedo[3u];
    f_total_radiance = vec4<f32>(_e65.x, _e65.y, _e65.z, _e67);
    return;
}

[[stage(fragment)]]
fn main([[builtin(position)]] gl_FragCoord: vec4<f32>) -> [[location(0)]] vec4<f32> {
    gl_FragCoord_1 = gl_FragCoord;
    main_1();
    let _e3 = f_total_radiance;
    return _e3;
}
