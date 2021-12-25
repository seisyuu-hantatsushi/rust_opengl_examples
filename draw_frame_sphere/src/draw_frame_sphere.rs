use std::ffi::{CStr,CString};
use std::{ptr, str, mem};
use std::{f64::consts::PI};
use gl;
use gl::types::*;
use linear_transform::{vector::*};
use graphic_math::graphic_math;

static VERTEX_SHADER_CODE: &'static str = include_str!("simple_viewport.vert");

static FRAGMENT_SHADER_CODE: &'static str = include_str!("simple.frag");

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

struct VertexArrayObjectContext {
    vao: GLuint,
    draw_mode: GLenum,
    count_of_draw_index: GLsizei
}

pub struct GlRender {
    shader_program: GLuint,
    vertex_array_object_contexts: Vec<VertexArrayObjectContext>
}

fn frame_sphere_vertices(radius:f64, slice:u32, stack:u32) -> (Vec<[f64;3]>,Vec<[u32;2]>) {
    let mut ps:Vec<[f64;3]> = Vec::new();
    let mut is:Vec<[u32;2]> = Vec::new();
    for j in 0 ..  stack+1 {
	if j == 0 {
	    ps.push([0.0, 0.0, radius]);
	}
	else if j == stack {
	    ps.push([0.0, 0.0, -radius]);
	}
	else {
	    let theta = PI*(j as f64)/((stack) as f64);
	    for i in 0..slice {
		let phi:f64 = 2.0*PI*(i as f64)/(slice as f64);
		ps.push([radius*theta.sin()*phi.cos(), radius*theta.sin()*phi.sin(), radius*theta.cos()]);
	    }
	    for i in 0 .. slice {
		is.push([1+slice*(j-1)+i%slice, 1+slice*(j-1)+(i+1)%slice]);
	    }
	}
    }

    for j in 0 .. stack {
	if j == 0 {
	    for i in 0 .. slice {
		is.push([0, 1+i%slice]);
	    }
	}
	else if j == stack-1 {
	    for i in 0 .. slice {
		is.push([ 1 + slice*(stack-2)+i , 1 + slice*(stack-1)]);
	    }
	}
	else {
	    for i in 0 .. slice {
		is.push([ 1 + slice*(j-1) + i , 1 + slice*j + i]);
	    }
	}
    }

    (ps,is)
}

static COORDINATE_AXES_VERTEX_DATA: [[GLfloat;3];6] = [
    [  2.0,  0.0,  0.0 ],
    [ -2.0,  0.0,  0.0 ],
    [  0.0,  2.0,  0.0 ],
    [  0.0, -2.0,  0.0 ],
    [  0.0,  0.0,  2.0 ],
    [  0.0,  0.0, -2.0 ],
];

static COORDINATE_AXES_INDEX_DATA: [[GLuint;2];3] = [
    [ 0, 1 ],
    [ 2, 3 ],
    [ 4, 5 ],
];

static COORDINATE_AXES_COLOR_DATA: [[GLfloat;4];6] = [
    [ 1.0, 0.0, 0.0, 0.0 ],
    [ 0.0, 0.0, 0.0, 0.0 ],
    [ 0.0, 1.0, 0.0, 0.0 ],
    [ 0.0, 0.0, 0.0, 0.0 ],
    [ 0.0, 0.0, 1.0, 0.0 ],
    [ 0.0, 0.0, 0.0, 0.0 ],
];

unsafe fn create_coordinate_axes_array(vao:GLuint) -> VertexArrayObjectContext {
    let position_location = 0;
    let color_location = 1;
    let vertex_vbo_index        = 0;
    let element_index_vbo_index = 1;
    let vertex_color_vbo_index  = 2;
    let mut vbos : [GLuint;3] = [0,0,0];

    gl::BindVertexArray(vao);
    gl::GenBuffers(3, &mut vbos[0]);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (COORDINATE_AXES_VERTEX_DATA.len() * 3 * mem::size_of::<GLfloat>()) as GLsizeiptr,
		   ptr::null(),
                   gl::STATIC_DRAW);
    {
	let mapped_buffer: *mut GLfloat = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GLfloat;
	let mut i:isize = 0;
	for p in COORDINATE_AXES_VERTEX_DATA.iter() {
	    let x: *mut GLfloat = mapped_buffer.offset(i+0);
	    let y: *mut GLfloat = mapped_buffer.offset(i+1);
	    let z: *mut GLfloat = mapped_buffer.offset(i+2);
	    *x = p[0];
	    *y = p[1];
	    *z = p[2];
	    i+=3;
	}
	gl::UnmapBuffer(gl::ARRAY_BUFFER);
    }

    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (COORDINATE_AXES_COLOR_DATA.len() * 4 * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   ptr::null(),
                   gl::STATIC_DRAW);
    {
	let mapped_buffer: *mut GLfloat = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GLfloat;
	for i in 0 ..  COORDINATE_AXES_COLOR_DATA.len() {
	    let r: *mut GLfloat = mapped_buffer.offset((4*i+0) as isize);
	    let g: *mut GLfloat = mapped_buffer.offset((4*i+1) as isize);
	    let b: *mut GLfloat = mapped_buffer.offset((4*i+2) as isize);
	    let a: *mut GLfloat = mapped_buffer.offset((4*i+3) as isize);
	    *r = COORDINATE_AXES_COLOR_DATA[i][0];
	    *g = COORDINATE_AXES_COLOR_DATA[i][1];
	    *b = COORDINATE_AXES_COLOR_DATA[i][2];
	    *a = COORDINATE_AXES_COLOR_DATA[i][3];
	}
	gl::UnmapBuffer(gl::ARRAY_BUFFER);
    }

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbos[element_index_vbo_index]);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   (COORDINATE_AXES_INDEX_DATA.len() * 2 * mem::size_of::<GLuint>()) as GLsizeiptr,
                   ptr::null(),
                   gl::STATIC_DRAW);
    {
	let mapped_buffer: *mut GLuint = gl::MapBuffer(gl::ELEMENT_ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GLuint;
	for i in 0 .. COORDINATE_AXES_INDEX_DATA.len() {
	    let s: *mut GLuint = mapped_buffer.offset((2*i+0) as isize);
	    let d: *mut GLuint = mapped_buffer.offset((2*i+1) as isize);
	    *s = COORDINATE_AXES_INDEX_DATA[i][0];
	    *d = COORDINATE_AXES_INDEX_DATA[i][1];
	}
	gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);
    }

    //simple_viewport.vertの変数"position"と頂点バッファを結びつける
    gl::EnableVertexAttribArray(position_location);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
    //shader側のメモリに配置したVertex情報のフォーマットを設定する.
    //今回は3次元座標なので,size=3
    gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

    //simple_viewport.vertの変数"vertexColor"と頂点での色情報バッファを結びつける.
    gl::EnableVertexAttribArray(color_location);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
    //shader側のメモリに配置したVertex情報のフォーマットを設定する.
    //今回はRGBAなので,size=4
    gl::VertexAttribPointer(color_location, 4, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

    gl::BindVertexArray(0);

    gl::DisableVertexAttribArray(color_location);
    gl::DisableVertexAttribArray(position_location);

    VertexArrayObjectContext {
	vao: vao,
	draw_mode: gl::LINES,
	count_of_draw_index: (COORDINATE_AXES_INDEX_DATA.len() * 2) as GLsizei
    }

}

unsafe fn create_sphere_array_object(vao:GLuint) ->  VertexArrayObjectContext {
    let mut vbos : [GLuint;3] = [0,0,0];
    let vertex_vbo_index        = 0;
    let element_index_vbo_index = 1;
    let vertex_color_vbo_index  = 2;
    let position_location = 0;
    let color_location = 1;

    let (circle_vertices, circle_indices) = frame_sphere_vertices(0.5, 24, 16);

    gl::BindVertexArray(vao);
    gl::GenBuffers(3, &mut vbos[0]);

    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (circle_vertices.len() * 3 * mem::size_of::<GLfloat>()) as GLsizeiptr,
		   ptr::null(),
                   gl::STATIC_DRAW);
    {
	let mapped_buffer: *mut GLfloat = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GLfloat;
	let mut i:isize = 0;
	for p in circle_vertices.iter() {
	    let x: *mut GLfloat = mapped_buffer.offset(i+0);
	    let y: *mut GLfloat = mapped_buffer.offset(i+1);
	    let z: *mut GLfloat = mapped_buffer.offset(i+2);
	    *x = p[0] as GLfloat;
	    *y = p[1] as GLfloat;
	    *z = p[2] as GLfloat;
	    i+=3;
	}
	gl::UnmapBuffer(gl::ARRAY_BUFFER);
    }

    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (circle_vertices.len() * 4 * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   ptr::null(),
                   gl::STATIC_DRAW);
    {
	let mapped_buffer: *mut GLfloat = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GLfloat;
	for i in 0 .. circle_vertices.len() {
	    let r: *mut GLfloat = mapped_buffer.offset((4*i+0) as isize);
	    let g: *mut GLfloat = mapped_buffer.offset((4*i+1) as isize);
	    let b: *mut GLfloat = mapped_buffer.offset((4*i+2) as isize);
	    let a: *mut GLfloat = mapped_buffer.offset((4*i+3) as isize);
	    *r = 1.0;
	    *g = 1.0;
	    *b = 0.0;
	    *a = 1.0;
	}
	gl::UnmapBuffer(gl::ARRAY_BUFFER);
    }

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbos[element_index_vbo_index]);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                   (circle_indices.len() * 2 * mem::size_of::<GLuint>()) as GLsizeiptr,
                       ptr::null(),
                   gl::STATIC_DRAW);
    {
	let mapped_buffer: *mut GLuint = gl::MapBuffer(gl::ELEMENT_ARRAY_BUFFER, gl::WRITE_ONLY) as *mut GLuint;
	for i in 0 .. circle_indices.len() {
	    let s: *mut GLuint = mapped_buffer.offset((2*i+0) as isize);
	    let d: *mut GLuint = mapped_buffer.offset((2*i+1) as isize);
	    *s = circle_indices[i][0];
	    *d = circle_indices[i][1];
	}
	gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);
    }

    //simple_viewport.vertの変数"position"と頂点バッファを結びつける.
    gl::EnableVertexAttribArray(position_location);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_vbo_index]);
    //shader側のメモリに配置したVertex情報のフォーマットを設定する.
    //今回は3次元座標なので,size=3
    gl::VertexAttribPointer(position_location, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

    //simple_viewport.vertの変数"vertexColor"と頂点での色情報バッファを結びつける.
    gl::EnableVertexAttribArray(color_location);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbos[vertex_color_vbo_index]);
    //shader側のメモリに配置したVertex情報のフォーマットを設定する.
    //今回はRGBAなので,size=4
    gl::VertexAttribPointer(color_location, 4, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());

    gl::BindVertexArray(0); //先にVAOを解く. でないと,ELEMENT_BUFFERとARRAY_BUFFERがVAOから外される.
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    gl::DisableVertexAttribArray(color_location);
    gl::DisableVertexAttribArray(position_location);

    VertexArrayObjectContext {
	vao: vao,
	draw_mode: gl::LINES,
	count_of_draw_index: (circle_indices.len() * 3) as GLsizei
    }
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
    let mut vao: [GLuint;2] = [0,0];
    let mut ctxs : Vec<VertexArrayObjectContext> = Vec::new();

    unsafe {
	gl::DeleteShader(fragment_shader);
	gl::DeleteShader(vertex_shader);

	gl::GenVertexArrays(2, &mut vao[0]);

	// 座標軸をかく
	ctxs.push(create_coordinate_axes_array(vao[0]));

	// 球を書いてみる
	ctxs.push(create_sphere_array_object(vao[1]));

    }

    GlRender {
	shader_program: shader_program,
	vertex_array_object_contexts: ctxs
    }
}

impl GlRender {
    pub fn render(&self, width:i32, height:i32) {
	//射影変換行列を計算する.
	let r:f64      = 4.0;
	let theta:f64  = PI*60.0/180.0;
	let phi:f64    = PI*45.0/180.0;
	let eye    = Vector3(r*theta.sin()*phi.cos(), r*theta.sin()*phi.sin(), r*theta.cos());
	let center = Vector3(0.0,0.0,0.0);
	let up     = Vector3(0.0,0.0,1.0);
	let lookat = graphic_math::look_at(eye,center,up);
	let aspect = (width as f64)/ (height as f64);
	let pers   = graphic_math::perspective(30.0, aspect, 1.0, 11.0);
	let mvp    = pers*lookat;
	unsafe {
	    let mvp_str = CString::new("mvp").unwrap_or_else(|_| panic!("failed to allocate string space"));
	    let mvp_str_ptr = mvp_str.as_ptr();
	    let mvp_location = gl::GetUniformLocation(self.shader_program, mvp_str_ptr);
	    gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
	    gl::Viewport(0, 0, width, height);
	    gl::UseProgram(self.shader_program);
	    gl::ProgramUniformMatrix4fv(self.shader_program, mvp_location, 1, gl::TRUE, mem::transmute(&mvp.serialize_f32()[0]));

	    gl::BindVertexArray(self.vertex_array_object_contexts[0].vao);
	    gl::DrawElements(self.vertex_array_object_contexts[0].draw_mode,
			     self.vertex_array_object_contexts[0].count_of_draw_index,
			     gl::UNSIGNED_INT, ptr::null());
	    gl::BindVertexArray(0);

	    gl::BindVertexArray(self.vertex_array_object_contexts[1].vao);
	    gl::DrawElements(self.vertex_array_object_contexts[1].draw_mode,
			     self.vertex_array_object_contexts[1].count_of_draw_index,
			     gl::UNSIGNED_INT, ptr::null());
	    gl::BindVertexArray(0);

	    gl::Flush();
	}
    }
}

