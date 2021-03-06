use std::ffi::{CStr,CString};
use std::{ptr, str, mem};
use gl;
use gl::types::*;
use linear_transform::{vector::*,matrix::{Matrix4x4}};
use graphic_math::graphic_math;

static VERTEX_SHADER_CODE: &'static str = include_str!("simple_viewport.vert");

static FRAGMENT_SHADER_CODE: &'static str = include_str!("simple.frag");

static CUBE_VERTEX_DATA: [GLfloat; 3*8] = [
    -1.0, -1.0,  -1.0,
     1.0, -1.0,  -1.0,
     1.0, -1.0,   1.0,
    -1.0, -1.0,   1.0,
    -1.0,  1.0,  -1.0,
     1.0,  1.0,  -1.0,
     1.0,  1.0,   1.0,
    -1.0,  1.0,   1.0
];

static CUBE_ELEMENT_INDEX: [GLuint; 2*12] = [
    0, 1,
    1, 2,
    2, 3,
    3, 0,
    0, 4,
    1, 5,
    2, 6,
    3, 7,
    4, 5,
    5, 6,
    6, 7,
    7, 4
];

static VERTEX_COLOR_DATA: [GLfloat; 4*8] = [
    1.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 0.0, 1.0,
    1.0, 0.0, 0.0, 1.0,
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
    let mut vbos : [GLuint;3] = [0,0,0];
    let vertex_vbo_index        = 0;
    let element_index_vbo_index = 1;
    let vertex_color_vbo_index  = 2;
    let position_location = 0;
    let color_location = 1;
    unsafe {
	gl::DeleteShader(fragment_shader);
	gl::DeleteShader(vertex_shader);

	gl::GenVertexArrays(1, &mut vao);
	gl::BindVertexArray(vao);

	gl::GenBuffers(3, &mut vbos[0]);

	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
	gl::BufferData(gl::ARRAY_BUFFER,
                       (CUBE_VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&CUBE_VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbos[element_index_vbo_index]);
	gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (CUBE_ELEMENT_INDEX.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                       mem::transmute(&CUBE_ELEMENT_INDEX[0]),
                       gl::STATIC_DRAW);

	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
	gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_COLOR_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_COLOR_DATA[0]),
                       gl::STATIC_DRAW);

	//simple_viewport.vert?????????"position"???????????????????????????????????????.
	gl::EnableVertexAttribArray(position_location);
	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
	//shader??????????????????????????????Vertex??????????????????????????????????????????.
	//?????????3?????????????????????,size=3
	gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

	//simple_viewport.vert?????????"vertexColor"??????????????????????????????????????????????????????.
	gl::EnableVertexAttribArray(color_location);
	gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
	//shader??????????????????????????????Vertex??????????????????????????????????????????.
	//?????????RGBA?????????,size=4
	gl::VertexAttribPointer(color_location, 4, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

	gl::BindVertexArray(0); //??????VAO?????????. ????????????,ELEMENT_BUFFER???ARRAY_BUFFER???VAO??????????????????.
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
	gl::BindBuffer(gl::ARRAY_BUFFER, 0);

	gl::DisableVertexAttribArray(color_location);
	gl::DisableVertexAttribArray(position_location);

    }

    GlRender {
	shader_program: shader_program,
	vertex_array_object: vao
    }
}

impl GlRender {
    pub fn render(&self) {
	//?????????????????????????????????.
	let r:f64      = 7.0;
	let theta:f64  = 45.0;
	let phi:f64    = 45.0;
	let eye    = Vector3(r*theta.sin()*phi.cos(),r*theta.sin()*phi.sin(),r*theta.cos());
	let center = Vector3(0.0,0.0,0.0);
	let up     = Vector3(0.0,1.0,0.0);
	let lookat = graphic_math::look_at(eye,center,up);
	let pers   = graphic_math::perspective(30.0, 1.0, 1.0, 11.0);
	let mvp    = pers*lookat;
	unsafe {
	    let mvp_str = CString::new("mvp").unwrap_or_else(|_| panic!("failed to allocate string space"));
	    let mvp_str_ptr = mvp_str.as_ptr();
	    let mvp_location = gl::GetUniformLocation(self.shader_program, mvp_str_ptr);
	    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
	    gl::UseProgram(self.shader_program);
	    gl::ProgramUniformMatrix4fv(self.shader_program, mvp_location, 1, gl::TRUE, mem::transmute(&mvp.serialize_f32()[0]));
	    gl::BindVertexArray(self.vertex_array_object);
	    gl::DrawElements(gl::LINES, 24, gl::UNSIGNED_INT, ptr::null());
	    gl::BindVertexArray(0);
	    gl::Flush();
	}
    }
}

