extern crate fnv;
extern crate opengl_es_rs as gles;
extern crate rand;
extern crate regex;

use rand::prelude::*;

pub use gles::es20::ffi::*;
pub use gles::consts::*;
pub use gles::types::*;

mod gl_attribute;
mod gl_buffer;
mod gl_enums;
mod gl_framebuffer;
mod gl_helpers;
mod gl_info;
mod gl_program;
mod gl_renderbuffer;
mod gl_texture;
mod gl_uniform;
mod gl_vertex;

pub use self::gl_attribute::GLAttribute;
pub use self::gl_buffer::GLBuffer;
pub use self::gl_enums::*;
pub use self::gl_framebuffer::GLFramebuffer;
pub use self::gl_helpers::*;
pub use self::gl_info::*;
pub use self::gl_program::*;
pub use self::gl_renderbuffer::GLRenderbuffer;
pub use self::gl_texture::GLTexture;
pub use self::gl_uniform::GLUniform;
pub use self::gl_vertex::GLVertex;


static SIMPLE_VERTEX_DATA: [f32; 16] = [
    //   position     uv
    1f32, 1f32, 1f32, 1f32, -1f32, 1f32, 0f32, 1f32, 1f32, -1f32, 1f32,
    0f32, -1f32, -1f32, 0f32, 0f32,
];

static SIMPLE_VERTEX: &'static str = "
#version 100
    uniform vec2 size;

    attribute vec2 position;
    attribute vec2 uv;

    varying vec2 v_uv;

    void main() {
        gl_Position = vec4(position * size * 0.5, 0, 1.0);
        v_uv = uv;
    }
";

#[cfg(any(target_os = "emscripten", target_os = "android"))]
static SIMPLE_FRAGMENT: &'static str = "
    #version 100
    precision mediump float;

    uniform float alpha;

    varying vec2 v_uv;

    void main() {
        gl_FragColor = vec4(v_uv, 1.0, alpha);
    }
";
#[cfg(not(any(target_os = "emscripten", target_os = "android")))]
static SIMPLE_FRAGMENT: &'static str = "
    #version 100
        precision mediump float;
    uniform float alpha;

    varying vec2 v_uv;

    void main() {
        gl_FragColor = vec4(v_uv, 1.0, alpha);
    }
";

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jlong, jstring};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_vampire_opengl_Rust_init(
        env: JNIEnv,
        _: JClass,
    ) -> jlong {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        //        let world = rust_greeting(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        //        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        //        let world_ptr = CString::from_raw(world);
        //        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        //        let res = &rust_opengl_backend_init();

        //        let res : *mut GLProgramWrapper = &mut (*rust_opengl_backend_init());

        let res: *mut GLProgramWrapper = Box::into_raw(rust_opengl_backend_init());

        res as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_vampire_opengl_Rust_draw(
        env: JNIEnv,
        _: JClass,
        handle: jlong,
    ) {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        //        let world = rust_greeting(env.get_string(java_pattern).expect("invalid pattern string").as_ptr());
        //        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        //        let world_ptr = CString::from_raw(world);
        //        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

//        rust_opengl_backend_draw(&mut (*(handle as *mut GLProgramWrapper)));

        //        init_env_rs();
    }
}

#[repr(C)]
pub struct GLProgramWrapper {
    program: GLProgram,
    timestamp: f32,
}

#[no_mangle]
pub extern "C" fn rust_opengl_backend_init() -> Box<GLProgramWrapper> {
    let gl_info = GLInfo::new();
    gl_set_defaults();

    let program = GLProgram::new(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
    let program_wrapper = GLProgramWrapper {
        program,
        timestamp: 0.1f32,
    };

    println!("\n -------- rust_opengl_backend_init -----------------");

//    unsafe {
//        let mut sampler: GLuint = 0;
//        glGenSamplers(1, &mut sampler);
//        println!("sampler id = {}", sampler);
//        let mut vao: GLuint = 0;
//        glGenVertexArrays(1, &mut vao);
//        println!("vertex array id = {}", vao);
//    }
//    println!("\n -------- rust_opengl_backend_es30_test -----------------");

    Box::new(program_wrapper)
}

//#[no_mangle]
//pub extern "C" fn rust_opengl_backend_draw(wrapper: &mut GLProgramWrapper) {
//    let program = &wrapper.program;
//    program.bind();
//
//    let buffer = GLBuffer::new(
//        BufferTarget::Array,
//        4,
//        Usage::StaticDraw,
//        &SIMPLE_VERTEX_DATA,
//    );
//
//    let mut vertex_array = GLVertexArray::new();
//
//    vertex_array.bind();
//    vertex_array.add_attribute(&buffer, program.get_attribute("position"), 0);
//    vertex_array.add_attribute(&buffer, program.get_attribute("uv"), 2);
//
//    vertex_array.enable_attributes();
//
//    program.get_uniform("alpha").set_1f(0.5_f32);
//
//    gl_set_viewport(0, 0, 750, 1334);
//
//    //    gl_set_clear_color(&[1.0, 0.0, 0.0, 1.0]);
//    gl_clear(true, true, true);
//
//    let mut size = [1f32; 2];
//    size[0] = 1_f32 + ((wrapper.timestamp * 0.005_f32).cos() * 0.5_f32);
//    size[1] = 1_f32 + ((wrapper.timestamp * 0.005_f32).sin() * 0.5_f32);
//    program.get_uniform("size").set_2f(&size);
//
//    program.get_uniform("alpha").set_1f(wrapper.timestamp);
//
//    gl_draw_arrays(DrawMode::TriangleStrip, 0, 4);
//
//    unsafe {
//        glFlush();
//    }
//
//    wrapper.timestamp += 0.1;
//    if wrapper.timestamp > 1.0 {
//        wrapper.timestamp = 0.0;
//    }
//    println!("\n -------- over -----------------");
//}
