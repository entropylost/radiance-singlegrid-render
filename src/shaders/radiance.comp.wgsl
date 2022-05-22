struct RadianceDirectionalUniforms {
    slope: f32;
    flags: u32;
    starting_radiance: vec3<f32>;
};

struct RadianceUniforms {
    inv_light_directions: f32;
    radiance_directional_uniforms: [[stride(32)]] array<RadianceDirectionalUniforms,64u>;
};

struct Workgroup {
    offset: i32;
    light_direction_index: u32;
};

struct Workgroups {
    workgroups: [[stride(8)]] array<Workgroup>;
};

struct GlobalUniforms {
    window_size: vec2<f32>;
};

[[group(2), binding(2)]]
var t_normal: texture_2d<f32>;
[[group(3), binding(0)]]
var t_total_radiance: texture_2d<f32>;
[[group(1), binding(0)]]
var<uniform> unnamed: RadianceUniforms;
[[group(3), binding(1)]]
var o_directional_radiance: texture_storage_2d_array<r32uint,write>;
[[group(1), binding(1)]]
var<storage, read_write> unnamed_1: Workgroups;
var<private> gl_WorkGroupID_1: vec3<u32>;
var<private> gl_LocalInvocationID_1: vec3<u32>;
[[group(0), binding(0)]]
var<uniform> unnamed_2: GlobalUniforms;
[[group(2), binding(0)]]
var t_albedo: texture_2d<f32>;
[[group(2), binding(1)]]
var t_radiance: texture_2d<f32>;

fn to_rgb9e5vf3_(v: ptr<function, vec3<f32>>) -> u32 {
    var clamped: vec3<f32>;
    var max_val: f32;
    var exponent: i32;
    var written_exponent: u32;
    var scale: f32;
    var mantissas: vec3<u32>;
    var encoded: u32;

    let _e51 = (*v);
    clamped = clamp(_e51, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(65408.0, 65408.0, 65408.0));
    let _e54 = clamped[0u];
    let _e56 = clamped[1u];
    let _e59 = clamped[2u];
    max_val = max(max(_e54, _e56), _e59);
    let _e61 = max_val;
    exponent = clamp((bitcast<i32>(extractBits(bitcast<u32>(_e61), bitcast<u32>(23), bitcast<u32>(8))) - 126), -15, 17);
    let _e69 = exponent;
    written_exponent = bitcast<u32>((_e69 + 15));
    let _e72 = exponent;
    scale = exp2(f32((9 - _e72)));
    let _e76 = clamped;
    let _e77 = scale;
    mantissas = vec3<u32>(((_e76 * _e77) + vec3<f32>(0.5, 0.5, 0.5)));
    let _e82 = mantissas[0u];
    let _e85 = mantissas[1u];
    let _e91 = mantissas[2u];
    let _e96 = written_exponent;
    encoded = ((((_e82 & 511u) | ((_e85 & 511u) << bitcast<u32>(9))) | ((_e91 & 511u) << bitcast<u32>(18))) | (_e96 << bitcast<u32>(27)));
    let _e100 = encoded;
    return _e100;
}

fn stepvi2vf2u1vf3_(position: ptr<function, vec2<i32>>, direction: ptr<function, vec2<f32>>, light_direction_index: ptr<function, u32>, radiance: ptr<function, vec3<f32>>) {
    var normal: vec2<f32>;
    var current_radiance: vec4<f32>;
    var alpha: f32;
    var light_in_direction: f32;
    var param: vec3<f32>;

    let _e52 = (*position);
    let _e53 = textureLoad(t_normal, _e52, 0);
    normal = _e53.xy;
    let _e55 = (*position);
    let _e56 = textureLoad(t_total_radiance, _e55, 0);
    current_radiance = _e56;
    let _e58 = current_radiance[3u];
    alpha = _e58;
    let _e60 = unnamed.inv_light_directions;
    light_in_direction = _e60;
    let _e61 = (*position);
    let _e62 = (*light_direction_index);
    let _e66 = vec3<i32>(_e61.x, _e61.y, bitcast<i32>(_e62));
    let _e67 = (*radiance);
    let _e68 = alpha;
    param = (_e67 * _e68);
    let _e70 = to_rgb9e5vf3_((&param));
    textureStore(o_directional_radiance, vec2<i32>(_e66.x, _e66.y), i32(_e66.z), vec4<u32>(_e70, 0u, 0u, 0u));
    let _e77 = normal;
    if (any((_e77 != vec2<f32>(0.0, 0.0)))) {
        let _e80 = normal;
        let _e81 = (*direction);
        let _e84 = light_in_direction;
        light_in_direction = (_e84 * (dot(_e80, _e81) * 6.2831854820251465));
    }
    let _e86 = (*radiance);
    let _e87 = alpha;
    let _e90 = current_radiance;
    let _e92 = light_in_direction;
    (*radiance) = ((_e86 * (1.0 - _e87)) + (_e90.xyz * max(_e92, 0.0)));
    return;
}

fn main_1() {
    var workgroup_: Workgroup;
    var total_offset: i32;
    var light_direction_index_1: u32;
    var uf: RadianceDirectionalUniforms;
    var slope: f32;
    var flags: u32;
    var radiance_1: vec3<f32>;
    var direction_1: vec2<f32>;
    var intersection_a: f32;
    var intersection_b: f32;
    var start: i32;
    var stop: i32;
    var i: i32;
    var position_1: vec2<i32>;
    var param_1: vec2<i32>;
    var param_2: vec2<f32>;
    var param_3: u32;
    var param_4: vec3<f32>;
    var direction_2: vec2<f32>;
    var intersection_a_1: f32;
    var intersection_b_1: f32;
    var start_1: i32;
    var stop_1: i32;
    var i_1: i32;
    var position_2: vec2<i32>;
    var param_5: vec2<i32>;
    var param_6: vec2<f32>;
    var param_7: u32;
    var param_8: vec3<f32>;
    var direction_3: vec2<f32>;
    var intersection_a_2: f32;
    var intersection_b_2: f32;
    var start_2: i32;
    var stop_2: i32;
    var i_2: i32;
    var position_3: vec2<i32>;
    var param_9: vec2<i32>;
    var param_10: vec2<f32>;
    var param_11: u32;
    var param_12: vec3<f32>;
    var direction_4: vec2<f32>;
    var intersection_a_3: f32;
    var intersection_b_3: f32;
    var start_3: i32;
    var stop_3: i32;
    var i_3: i32;
    var position_4: vec2<i32>;
    var param_13: vec2<i32>;
    var param_14: vec2<f32>;
    var param_15: u32;
    var param_16: vec3<f32>;

    let _e95 = gl_WorkGroupID_1[0u];
    let _e98 = unnamed_1.workgroups[_e95];
    workgroup_.offset = _e98.offset;
    workgroup_.light_direction_index = _e98.light_direction_index;
    let _e104 = workgroup_.offset;
    let _e106 = gl_LocalInvocationID_1[0u];
    total_offset = (_e104 + bitcast<i32>(_e106));
    let _e110 = workgroup_.light_direction_index;
    light_direction_index_1 = _e110;
    let _e111 = light_direction_index_1;
    let _e114 = unnamed.radiance_directional_uniforms[_e111];
    uf.slope = _e114.slope;
    uf.flags = _e114.flags;
    uf.starting_radiance = _e114.starting_radiance;
    let _e122 = uf.slope;
    slope = _e122;
    let _e124 = uf.flags;
    flags = _e124;
    let _e126 = uf.starting_radiance;
    radiance_1 = _e126;
    let _e127 = flags;
    if (((_e127 & 1u) == 0u)) {
        let _e130 = flags;
        if (((_e130 & 2u) == 0u)) {
            let _e133 = slope;
            direction_1 = normalize(vec2<f32>(1.0, _e133));
            let _e136 = total_offset;
            let _e139 = slope;
            intersection_a = (-(f32(_e136)) / _e139);
            let _e143 = unnamed_2.window_size[1u];
            let _e144 = total_offset;
            let _e147 = slope;
            intersection_b = ((_e143 - f32(_e144)) / _e147);
            let _e149 = intersection_a;
            let _e150 = intersection_b;
            start = max(0, i32(min(_e149, _e150)));
            let _e156 = unnamed_2.window_size[0u];
            let _e158 = intersection_a;
            let _e159 = intersection_b;
            stop = min(i32(_e156), i32(ceil(max(_e158, _e159))));
            let _e164 = start;
            i = _e164;
            loop {
                let _e165 = i;
                let _e166 = stop;
                if ((_e165 < _e166)) {
                    let _e168 = i;
                    let _e169 = total_offset;
                    let _e170 = i;
                    let _e172 = slope;
                    position_1 = vec2<i32>(_e168, (_e169 + i32(floor((f32(_e170) * _e172)))));
                    let _e178 = position_1;
                    param_1 = _e178;
                    let _e179 = direction_1;
                    param_2 = _e179;
                    let _e180 = light_direction_index_1;
                    param_3 = _e180;
                    let _e181 = radiance_1;
                    param_4 = _e181;
                    stepvi2vf2u1vf3_((&param_1), (&param_2), (&param_3), (&param_4));
                    let _e182 = param_4;
                    radiance_1 = _e182;
                    continue;
                } else {
                    break;
                }
                continuing {
                    let _e183 = i;
                    i = (_e183 + 1);
                }
            }
        } else {
            let _e185 = slope;
            direction_2 = normalize(vec2<f32>(_e185, 1.0));
            let _e188 = total_offset;
            let _e191 = slope;
            intersection_a_1 = (-(f32(_e188)) / _e191);
            let _e195 = unnamed_2.window_size[0u];
            let _e196 = total_offset;
            let _e199 = slope;
            intersection_b_1 = ((_e195 - f32(_e196)) / _e199);
            let _e201 = intersection_a_1;
            let _e202 = intersection_b_1;
            start_1 = max(0, i32(min(_e201, _e202)));
            let _e208 = unnamed_2.window_size[1u];
            let _e210 = intersection_a_1;
            let _e211 = intersection_b_1;
            stop_1 = min(i32(_e208), i32(ceil(max(_e210, _e211))));
            let _e216 = start_1;
            i_1 = _e216;
            loop {
                let _e217 = i_1;
                let _e218 = stop_1;
                if ((_e217 < _e218)) {
                    let _e220 = total_offset;
                    let _e221 = i_1;
                    let _e223 = slope;
                    let _e228 = i_1;
                    position_2 = vec2<i32>((_e220 + i32(floor((f32(_e221) * _e223)))), _e228);
                    let _e230 = position_2;
                    param_5 = _e230;
                    let _e231 = direction_2;
                    param_6 = _e231;
                    let _e232 = light_direction_index_1;
                    param_7 = _e232;
                    let _e233 = radiance_1;
                    param_8 = _e233;
                    stepvi2vf2u1vf3_((&param_5), (&param_6), (&param_7), (&param_8));
                    let _e234 = param_8;
                    radiance_1 = _e234;
                    continue;
                } else {
                    break;
                }
                continuing {
                    let _e235 = i_1;
                    i_1 = (_e235 + 1);
                }
            }
        }
    } else {
        let _e237 = flags;
        if (((_e237 & 2u) == 0u)) {
            let _e240 = slope;
            direction_3 = -(normalize(vec2<f32>(1.0, _e240)));
            let _e244 = total_offset;
            let _e247 = slope;
            intersection_a_2 = (-(f32(_e244)) / _e247);
            let _e251 = unnamed_2.window_size[1u];
            let _e252 = total_offset;
            let _e255 = slope;
            intersection_b_2 = ((_e251 - f32(_e252)) / _e255);
            let _e257 = intersection_a_2;
            let _e258 = intersection_b_2;
            start_2 = max(0, i32(min(_e257, _e258)));
            let _e264 = unnamed_2.window_size[0u];
            let _e267 = intersection_a_2;
            let _e268 = intersection_b_2;
            stop_2 = min(i32((_e264 - 1.0)), i32(ceil(max(_e267, _e268))));
            let _e273 = stop_2;
            i_2 = _e273;
            loop {
                let _e274 = i_2;
                let _e275 = start_2;
                if ((_e274 >= _e275)) {
                    let _e277 = i_2;
                    let _e278 = total_offset;
                    let _e279 = i_2;
                    let _e281 = slope;
                    position_3 = vec2<i32>(_e277, (_e278 + i32(floor((f32(_e279) * _e281)))));
                    let _e287 = position_3;
                    param_9 = _e287;
                    let _e288 = direction_3;
                    param_10 = _e288;
                    let _e289 = light_direction_index_1;
                    param_11 = _e289;
                    let _e290 = radiance_1;
                    param_12 = _e290;
                    stepvi2vf2u1vf3_((&param_9), (&param_10), (&param_11), (&param_12));
                    let _e291 = param_12;
                    radiance_1 = _e291;
                    continue;
                } else {
                    break;
                }
                continuing {
                    let _e292 = i_2;
                    i_2 = (_e292 - 1);
                }
            }
        } else {
            let _e294 = slope;
            direction_4 = -(normalize(vec2<f32>(_e294, 1.0)));
            let _e298 = total_offset;
            let _e301 = slope;
            intersection_a_3 = (-(f32(_e298)) / _e301);
            let _e305 = unnamed_2.window_size[0u];
            let _e306 = total_offset;
            let _e309 = slope;
            intersection_b_3 = ((_e305 - f32(_e306)) / _e309);
            let _e311 = intersection_a_3;
            let _e312 = intersection_b_3;
            start_3 = max(0, i32(min(_e311, _e312)));
            let _e318 = unnamed_2.window_size[1u];
            let _e321 = intersection_a_3;
            let _e322 = intersection_b_3;
            stop_3 = min(i32((_e318 - 1.0)), i32(ceil(max(_e321, _e322))));
            let _e327 = stop_3;
            i_3 = _e327;
            loop {
                let _e328 = i_3;
                let _e329 = start_3;
                if ((_e328 >= _e329)) {
                    let _e331 = total_offset;
                    let _e332 = i_3;
                    let _e334 = slope;
                    let _e339 = i_3;
                    position_4 = vec2<i32>((_e331 + i32(floor((f32(_e332) * _e334)))), _e339);
                    let _e341 = position_4;
                    param_13 = _e341;
                    let _e342 = direction_4;
                    param_14 = _e342;
                    let _e343 = light_direction_index_1;
                    param_15 = _e343;
                    let _e344 = radiance_1;
                    param_16 = _e344;
                    stepvi2vf2u1vf3_((&param_13), (&param_14), (&param_15), (&param_16));
                    let _e345 = param_16;
                    radiance_1 = _e345;
                    continue;
                } else {
                    break;
                }
                continuing {
                    let _e346 = i_3;
                    i_3 = (_e346 - 1);
                }
            }
        }
    }
    return;
}

[[stage(compute), workgroup_size(16, 1, 1)]]
fn main([[builtin(workgroup_id)]] gl_WorkGroupID: vec3<u32>, [[builtin(local_invocation_id)]] gl_LocalInvocationID: vec3<u32>) {
    gl_WorkGroupID_1 = gl_WorkGroupID;
    gl_LocalInvocationID_1 = gl_LocalInvocationID;
    main_1();
}
