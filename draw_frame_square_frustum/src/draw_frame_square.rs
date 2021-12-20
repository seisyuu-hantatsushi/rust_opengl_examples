use std::ffi::{CStr,CString};
use std::{ptr, str, mem};
use gl;
use gl::types::*;
use linear_transform::{vector::*,matrix::{Matrix4x4}};
use graphic_math::graphic_math;

static VERTEX_SHADER_CODE: &'static str = include_str!("simple_viewport.vert");

static FRAGMENT_SHADER_CODE: &'static str = include_str!("simple.frag");

static VERTEX_DATA: [GLfloat; 12] = [
     0.9,   0.9,  -1.0,
    -0.9,   0.9,  -1.0,
    -0.9,  -0.9,  -1.0,
     0.9,  -0.9,  -1.0,
];

static VERTEX_COLOR_DATA: [GLfloat; 16] = [
    1.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 0.0, 1.0,
];

fn compile_shader(shader_code: &str, shader_type: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(shader_type);
        // Attempt to compile the shader
        let c_str = CString::new(shader_code.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len - 1) as usize); // -1 removes null terminator
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            println!("{}", str::from_utf8(mem::transmute(buf.as_slice())).ok().expect("Failed to transmute shader error string!"));
        }
    }
    shader
}

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len - 1) as usize); // -1 removes null terminator
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            println!("{}", str::from_utf8(mem::transmute(buf.as_slice())).ok().expect("Failed to transmute program error string!"));
        }
    }
    program
}

pub struct GlRender {
    shader_program: GLuint,
    vertex_array_object: GLuint
}

pub fn create_glrender<F>(loadfn:F) -> GlRender
where
    F:FnMut(&str) -> *const std::os::raw::c_void,
{

    gl::load_with(loadfn);

    unsafe {
	println!("Open GL version:{}",
		 CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_string_lossy().into_owned());
	println!("Shading lang version:{}",
		 CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8).to_string_lossy().into_owned());
    }

    let vertex_shader = compile_shader(VERTEX_SHADER_CODE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER_CODE, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);
    let mut vao = 0;
    let mut vbos : [GLuint;2] = [0,0];
    let vertex_vbo_index = 0;
    let vertex_color_vbo_index = 1;

    unsafe {
	gl::DeleteShader(fragment_shader);
	gl::DeleteShader(vertex_shader);

	gl::GenVertexArrays(1, &mut vao);
	gl::BindVertexArray(vao);

	gl::GenBuffers(2, &mut vbos[0]);

	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
	gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
	gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_COLOR_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_COLOR_DATA[0]),
                       gl::STATIC_DRAW);

	//simple_viewport.vertの変数"position"と頂点バッファを結びつける.
	{
	    let location = 0;
	    gl::EnableVertexAttribArray(location);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
	    //shader側のメモリに配置したVertex情報のフォーマットを設定する.
	    //今回は3次元座標なので,size=3
	    gl::VertexAttribPointer(location, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
	}

	//simple_viewport.vertの変数"vertexColor"と頂点での色情報バッファを結びつける.
	{
	    let location = 1;
	    gl::EnableVertexAttribArray(location);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
	    //shader側のメモリに配置したVertex情報のフォーマットを設定する.
	    //今回はRGBAなので,size=4
	    gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
	}
    }

    GlRender {
	shader_program: shader_program,
	vertex_array_object: vao
    }
}

impl GlRender {
    pub fn render(&self) {

	//射影変換行列を計算する.
	//let mvp = Matrix4x4::identity();
	let mvp = graphic_math::frustum(-1.0, 1.0, -1.0, 1.0, 0.1, -2.0);
	unsafe {
	    gl::ProgramUniformMatrix4fv(self.shader_program, 2, 1, gl::TRUE, mem::transmute(&mvp.serialize_f32()[0]));
	    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
	    gl::UseProgram(self.shader_program);
	    gl::BindVertexArray(self.vertex_array_object);
	    gl::DrawArrays(gl::LINE_LOOP, 0, 4);
	}
    }
}

