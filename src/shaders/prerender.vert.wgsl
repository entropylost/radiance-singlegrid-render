struct gl_PerVertex {
    [[builtin(position)]] gl_Position: vec4<f32>;
};

struct GlobalUniforms {
    window_size: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] gl_Position: vec4<f32>;
    [[location(0)]] member: vec4<f32>;
    [[location(1)]] member_1: vec4<f32>;
    [[location(2)]] member_2: vec2<f32>;
};

var<private> perVertexStruct: gl_PerVertex = gl_PerVertex(vec4<f32>(0.0, 0.0, 0.0, 1.0), );
var<private> position_1: vec2<f32>;
[[group(0), binding(0)]]
var<uniform> unnamed: GlobalUniforms;
var<private> o_albedo_lin: vec4<f32>;
var<private> albedo_lin_1: vec4<f32>;
var<private> o_radiance_lin: vec4<f32>;
var<private> radiance_lin_1: vec4<f32>;
var<private> o_normal: vec2<f32>;
var<private> normal_1: vec2<f32>;

fn main_1() {
    let _e19 = position_1;
    let _e21 = unnamed.window_size;
    let _e24 = (((_e19 / _e21) * 2.0) - vec2<f32>(1.0, 1.0));
    perVertexStruct.gl_Position = vec4<f32>(_e24.x, _e24.y, 0.0, 1.0);
    let _e29 = albedo_lin_1;
    o_albedo_lin = _e29;
    let _e30 = radiance_lin_1;
    o_radiance_lin = _e30;
    let _e31 = normal_1;
    o_normal = _e31;
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>, [[location(2)]] albedo_lin: vec4<f32>, [[location(3)]] radiance_lin: vec4<f32>, [[location(1)]] normal: vec2<f32>) -> VertexOutput {
    position_1 = position;
    albedo_lin_1 = albedo_lin;
    radiance_lin_1 = radiance_lin;
    normal_1 = normal;
    main_1();
    let _e14 = perVertexStruct.gl_Position.y;
    perVertexStruct.gl_Position.y = -(_e14);
    let _e16 = perVertexStruct.gl_Position;
    let _e17 = o_albedo_lin;
    let _e18 = o_radiance_lin;
    let _e19 = o_normal;
    return VertexOutput(_e16, _e17, _e18, _e19);
}
