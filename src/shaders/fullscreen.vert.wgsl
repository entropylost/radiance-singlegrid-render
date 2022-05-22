struct gl_PerVertex {
    [[builtin(position)]] gl_Position: vec4<f32>;
};

struct GlobalUniforms {
    window_size: vec2<f32>;
};

var<private> perVertexStruct: gl_PerVertex = gl_PerVertex(vec4<f32>(0.0, 0.0, 0.0, 1.0), );
var<private> position_1: vec2<f32>;
[[group(0), binding(0)]]
var<uniform> unnamed: GlobalUniforms;

fn main_1() {
    let _e11 = position_1;
    perVertexStruct.gl_Position = vec4<f32>(_e11.x, _e11.y, 0.0, 1.0);
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>) -> [[builtin(position)]] vec4<f32> {
    position_1 = position;
    main_1();
    let _e5 = perVertexStruct.gl_Position.y;
    perVertexStruct.gl_Position.y = -(_e5);
    let _e7 = perVertexStruct.gl_Position;
    return _e7;
}
