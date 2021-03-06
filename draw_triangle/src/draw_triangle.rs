use std::ffi::{CStr,CString};
use std::{ptr, str, mem};
use gl;
use gl::types::*;

static VERTEX_SHADER_CODE: &'static str =
    r#"#version 150
    in vec2 position;
    in vec3 color;
    out vec3 frag_color;
    void main() {
       gl_Position = vec4(position, 0.0, 1.0);
       frag_color = color;
    }"#;

static FRAGMENT_SHADER_CODE: &'static str =
    r#"#version 150
    in vec3 frag_color;
    out vec4 out_color;
    void main() {
       out_color = vec4(frag_color, 1.0);
    }"#;

static VERTEX_DATA: [GLfloat; 6] = [
    0.0,    0.5,
    0.5,   -0.5,
   -0.5,   -0.5
];

static FRAGMENT_COLOR_DATA: [GLfloat; 9] = [
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0
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
    let color_vbo_index  = 1;

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

	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[color_vbo_index]);
	gl::BufferData(gl::ARRAY_BUFFER,
                       (FRAGMENT_COLOR_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&FRAGMENT_COLOR_DATA[0]),
                       gl::STATIC_DRAW);

	let out_color_str = CString::new("out_color").unwrap_or_else(|_| panic!("failed to allocate string space"));
	let out_color_str_ptr = out_color_str.as_ptr();
	gl::BindFragDataLocation(shader_program, 0, out_color_str_ptr);

	let position_str = CString::new("position").unwrap_or_else(|_| panic!("failed to allocate string space"));
	let position_str_ptr = position_str.as_ptr();
	{
	    let location : GLuint  = gl::GetAttribLocation(shader_program, position_str_ptr) as GLuint;
	    gl::EnableVertexAttribArray(location);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
	    gl::VertexAttribPointer(location, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
	}

	let in_color_str = CString::new("color").unwrap_or_else(|_| panic!("failed to allocate string space"));
	let in_color_str_ptr = in_color_str.as_ptr();
	{
	    let location : GLuint  = gl::GetAttribLocation(shader_program, in_color_str_ptr) as GLuint;
	    gl::EnableVertexAttribArray(location);
	    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[color_vbo_index]);
	    gl::VertexAttribPointer(location, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
	}
    }

    GlRender {
	shader_program: shader_program,
	vertex_array_object: vao
    }
}

impl GlRender {
    pub fn render(&self) {
	unsafe {
	    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
	    gl::UseProgram(self.shader_program);
	    gl::BindVertexArray(self.vertex_array_object);
	    gl::DrawArrays(gl::TRIANGLES, 0, 3);
	}
    }
}
