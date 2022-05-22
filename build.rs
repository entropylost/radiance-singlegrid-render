use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use glob::glob;
use naga::back::wgsl::{write_string, WriterFlags};
use naga::front::spv::{Options, Parser};
use naga::valid::{Capabilities, ValidationFlags, Validator};
use shaderc::{
    CompileOptions, Compiler, IncludeCallbackResult, IncludeType, ResolvedInclude, ShaderKind,
};

fn include_callback(
    filename: &str,
    _ty: IncludeType,
    _origin_filename: &str,
    depth: usize,
) -> IncludeCallbackResult {
    if depth > 10 {
        panic!("Max include depth exceeded");
    }
    let mut path = Path::new("src/shaders/").to_path_buf();
    path.push(filename);
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
            let resolved_name = path
                .into_os_string()
                .into_string()
                .expect("Path contains invalid characters");
            Ok(ResolvedInclude {
                resolved_name,
                content,
            })
        }
        Err(err) => Err(err.to_string()),
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/shaders/");

    let mut compiler = Compiler::new().unwrap();
    let mut options = CompileOptions::new().unwrap();
    // options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    options.set_target_spirv(shaderc::SpirvVersion::V1_3);
    options.set_include_callback(include_callback);

    let spv_options = Options::default();

    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::empty());
    let mut compile_glsl = |path: PathBuf, kind| {
        println!("Compiling {:?}", path);

        let input = read_to_string(path.clone()).unwrap();
        let result = compiler
            .compile_into_spirv(
                &input,
                kind,
                path.file_name().unwrap().to_str().unwrap(),
                "main",
                Some(&options),
            )
            .unwrap();

        let module = Parser::new(result.as_binary().iter().cloned(), &spv_options)
            .parse()
            .unwrap();
        let module_info = validator.validate(&module).unwrap();
        let data = write_string(&module, &module_info, WriterFlags::empty()).unwrap();

        std::fs::write(
            path.with_extension(path.extension().unwrap().to_str().unwrap().to_owned() + ".wgsl"),
            data,
        )
        .unwrap();
    };

    for path in glob("src/shaders/*.frag").unwrap().flatten() {
        compile_glsl(path, ShaderKind::Fragment);
    }

    for path in glob("src/shaders/*.vert").unwrap().flatten() {
        compile_glsl(path, ShaderKind::Vertex);
    }

    for path in glob("src/shaders/*.comp").unwrap().flatten() {
        compile_glsl(path, ShaderKind::Compute);
    }
}
