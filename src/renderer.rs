#![macro_escape]

extern crate gl;
use gl::types::*;
use std::mem;
use std::collections::HashMap;
use std::ffi::{ CString, CStr };
use std::ptr;

use vecmath::*;

#[macro_export]
macro_rules! GL {
	($fun:ident ( $($arg:expr),*)) => {{
		unsafe {
			let result = ::gl::$fun( $($arg),* );
			let err = ::gl::GetError();
			if err != ::gl::NO_ERROR {
				let err_str = match err {
					::gl::INVALID_OPERATION => "Invalid Operation",
					::gl::INVALID_ENUM => "Invalid Enum",
					::gl::INVALID_VALUE => "Invalid Value",
					::gl::OUT_OF_MEMORY => "Out Of Memory",
					::gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid Framebuffer Operation",
					_ => "Unknown Error"
				};
				panic!("OpenGL Error ({}): {}\n\tFile:{}",
					err, err_str, line!()
				);
			}
			result
		}
	}};
}

pub struct Uniform {
	loc: i32
}

pub trait Setter<T> {
	fn set(&self, val: T);
}

impl Setter<i32> for Uniform {
	fn set(&self, val: i32) {
		GL!(Uniform1i(self.loc, val));
	}
}

impl Setter<f32> for Uniform {
	fn set(&self, val: f32) {
		GL!(Uniform1f(self.loc, val));
	}
}

impl Setter<Vec2> for Uniform {
	fn set(&self, val: Vec2) {
		GL!(Uniform2f(self.loc, val.x, val.y));
	}
}

impl Setter<Vec3> for Uniform {
	fn set(&self, val: Vec3) {
		GL!(Uniform3f(self.loc, val.x, val.y, val.z));
	}
}

impl Setter<Vec4> for Uniform {
	fn set(&self, val: Vec4) {
		GL!(Uniform4f(self.loc, val.x, val.y, val.z, val.w));
	}
}

impl Setter<Mat4> for Uniform {
	fn set(&self, val: Mat4) {
		GL!(UniformMatrix4fv(self.loc, 1, gl::TRUE, val.as_ptr()))
	}
}

pub struct Shader {
	program: u32,
	uniforms: HashMap<String, i32>
}

impl Drop for Shader {
	fn drop(&mut self) {
		if self.program > 0 {
			GL!(DeleteProgram(self.program));
		}
	}
}

impl Shader {
	pub fn new() -> Shader {
		Shader {
			program: GL!(CreateProgram()),
			uniforms: HashMap::new()
		}
	}

	pub fn add_shader(&mut self, src: &str, ty: GLenum) {
		let shader = match Shader::create_shader(src, ty) {
			None => panic!("Invalid Shader."),
			Some(s) => s
		};
		GL!(AttachShader(self.program, shader));
		GL!(DeleteShader(shader));
	}

	pub fn link(&mut self) {
		GL!(LinkProgram(self.program));

		let mut status = 0i32;
		GL!(GetProgramiv(self.program, gl::LINK_STATUS, &mut status));
		if status == 0 {
			panic!("Could not link program.");
		}
	}

	pub fn get_uniform_location(&mut self, name: &str) -> i32 {
		if !self.uniforms.contains_key(name) {
			let cstr = CString::new(name).unwrap();
			let loc = GL!(GetUniformLocation(self.program, cstr.as_ptr()));
			if loc > -1 {
				self.uniforms.insert(name.to_owned(), loc);
			} else {
				panic!("Uniform not found! \"{}\"", name);
			}
		}
		match self.uniforms.get(name) {
			Some(loc) => { *loc },
			None => { -1 }
		}
	}

	pub fn get(&mut self, uniform_name: &str) -> Option<Uniform> {
		let loc = self.get_uniform_location(uniform_name);
		if loc == -1 {
			return None;
		}
		Some(Uniform { loc: loc })
	}

	pub fn bind(&self) {
		GL!(UseProgram(self.program));
	}

	pub fn unbind(&self) {
		GL!(UseProgram(0));
	}

	fn create_shader(src: &str, ty: GLenum) -> Option<u32> {
		let shader = GL!(CreateShader(ty));
		unsafe {
			let c_str = CString::new(src).unwrap();
			GL!(ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null()));
			GL!(CompileShader(shader));

			let mut status = 0i32;
			GL!(GetShaderiv(shader, gl::COMPILE_STATUS, &mut status));
			if status == 0 {
				let mut buf = [0u8; 1024];
				let mut len = 0i32;
				GL!(GetShaderInfoLog(shader, buf.len() as i32, &mut len, buf.as_mut_ptr() as *mut _));

				println!("{}", CStr::from_bytes_with_nul_unchecked(&buf[..len as usize]).to_str().unwrap());
				return None;
			}
		}
		Some(shader)
	}
}

#[derive(Debug, Clone)]
pub struct VertexAttribute {
	comps: i32,
	norm: bool
}

impl VertexAttribute {
	pub fn new(components: i32, normalized: bool) -> VertexAttribute {
		VertexAttribute { comps: components, norm: normalized }
	}
}

#[derive(Debug, Clone)]
pub struct Model {
	vertices: Vec<f32>,
	indices: Vec<u16>,
	vbo: u32,
	vao: u32,
	ibo: u32,
	prevVBO: u32,
	prevIBO: u32
}

impl Model {
	pub fn new(fmt: &[VertexAttribute], instanced: bool) -> Model {
		let mut vao = 0;
		let mut vbo = 0;
		let mut ibo = 0;
		GL!(GenVertexArrays(1, &mut vao));
		GL!(GenBuffers(1, &mut vbo));
		GL!(GenBuffers(1, &mut ibo));

		GL!(BindVertexArray(vao));
		GL!(BindBuffer(gl::ARRAY_BUFFER, vbo));

		let mut stride = 0;
		for attr in fmt.iter().cloned() {
			stride += attr.comps * 4;
		}
		
		let mut off = 0i32;
		let mut last = 0u32;
		for (i, attr) in fmt.iter().cloned().enumerate() {
			GL!(EnableVertexAttribArray(i as u32));
			GL!(VertexAttribPointer(
				i as u32,
				attr.comps,
				gl::FLOAT,
				if attr.norm { gl::TRUE } else { gl::FALSE },
				stride,
				off as *const _
			));
			off += attr.comps * mem::size_of::<f32>() as i32;
			last = i as u32;
		}
		
		if instanced {
			let sz = mem::size_of::<Mat4>();
			GL!(EnableVertexAttribArray(last + 1));
			GL!(VertexAttribPointer(last + 1, 4, gl::FLOAT,	gl::FALSE, sz as _,	0 as *const _));
			GL!(EnableVertexAttribArray(last + 2));
			GL!(VertexAttribPointer(last + 2, 4, gl::FLOAT,	gl::FALSE, sz as _,	12 as *const _));
			GL!(EnableVertexAttribArray(last + 3));
			GL!(VertexAttribPointer(last + 3, 4, gl::FLOAT,	gl::FALSE, sz as _,	24 as *const _));
			GL!(EnableVertexAttribArray(last + 4));
			GL!(VertexAttribPointer(last + 4, 4, gl::FLOAT,	gl::FALSE, sz as _,	36 as *const _));

			GL!(VertexAttribDivisor(last + 1, 1));
			GL!(VertexAttribDivisor(last + 2, 1));
			GL!(VertexAttribDivisor(last + 3, 1));
			GL!(VertexAttribDivisor(last + 4, 1));
		}

		GL!(BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo));

		GL!(BindVertexArray(0));

		Model {
			vertices: Vec::new(),
			indices: Vec::new(),
			prevVBO: 0,
			prevIBO: 0,
			vbo: vbo,
			ibo: ibo,
			vao: vao
		}
	}

	pub fn from(vertices: &[f32], indices: &[u16], fmt: &[VertexAttribute], instanced: bool) -> Model {
		let mut m = Model::new(fmt, instanced);
		m.add_data(vertices, indices);
		m.flush();
		m
	}

	pub fn concat(a: Model, b: Model, fmt: &[VertexAttribute]) -> Model {
		let mut m = Model::new(fmt, false);
		let mut verts = Vec::new();
		let mut inds = Vec::new();
		verts.extend(a.vertices.clone());
		verts.extend(b.vertices.clone());
		inds.extend(a.indices.clone());
		inds.extend(b.indices.clone());
		m.add_data(verts.as_slice(), inds.as_slice());
		m.flush();
		m
	}

	pub fn add_data(&mut self, vertices: &[f32], indices: &[u16]) {
		self.vertices.extend(vertices.to_vec());
		self.indices.extend(indices.to_vec());
	}

	pub fn flush(&mut self) {
		GL!(BindBuffer(gl::ARRAY_BUFFER, self.vbo));
		if self.vertices.len() > self.prevVBO as usize {
			GL!(BufferData(
				gl::ARRAY_BUFFER,
				(self.vertices.len() * mem::size_of::<f32>()) as _,
				self.vertices.as_ptr() as *const _,
				gl::DYNAMIC_DRAW
			));
			self.prevVBO = self.vertices.len() as u32;
		} else {
			GL!(BufferSubData(
				gl::ARRAY_BUFFER,
				0,
				(self.vertices.len() * mem::size_of::<f32>()) as _,
				self.vertices.as_ptr() as *const _
			));
		}

		GL!(BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo));
		if self.indices.len() > self.prevIBO as usize {
			GL!(BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(self.indices.len() * mem::size_of::<u16>()) as _,
				self.indices.as_ptr() as *const _,
				gl::DYNAMIC_DRAW
			));
			self.prevIBO = self.indices.len() as u32;
		} else {
			GL!(BufferSubData(
				gl::ELEMENT_ARRAY_BUFFER,
				0,
				(self.indices.len() * mem::size_of::<u16>()) as _,
				self.indices.as_ptr() as *const _
			));
		}
	}

	pub fn draw(&self, prim: GLenum) {
		GL!(BindVertexArray(self.vao));
		GL!(DrawElements(
			prim,
			self.prevIBO as i32,
			gl::UNSIGNED_SHORT,
			0 as *const _
		));
		GL!(BindVertexArray(0));
	}

	pub fn draw_instanced(&self, prim: GLenum, amount: i32) {
		GL!(BindVertexArray(self.vao));
		GL!(DrawElementsInstanced(
			prim,
			self.prevIBO as i32,
			gl::UNSIGNED_SHORT,
			0 as *const _,
			amount
		));
		GL!(BindVertexArray(0));
	}

	pub fn free(&mut self) {
		if self.vbo > 0 {
			GL!(DeleteBuffers(1, &mut self.vbo));
			GL!(DeleteBuffers(1, &mut self.ibo));
			GL!(DeleteVertexArrays(1, &mut self.vao));
		}
	}
}

#[derive(Debug, Clone)]
struct TModel {
	transform: Mat4,
	model: Model
}

pub struct Renderer {
	instance_mats: u32
}

impl Renderer {
	pub fn new() -> Renderer {
		let mut instance_mats = 0;
		let max_instances = 100000;

		GL!(GenBuffers(1, &mut instance_mats));
		GL!(BindBuffer(gl::ARRAY_BUFFER, instance_mats));
		GL!(BufferData(gl::ARRAY_BUFFER, (max_instances * mem::size_of::<Mat4>()) as _, ptr::null(), gl::DYNAMIC_DRAW));
		GL!(BindBuffer(gl::ARRAY_BUFFER, 0));

		GL!(Enable(gl::DEPTH_TEST));
		GL!(Enable(gl::CULL_FACE));
		GL!(FrontFace(gl::CCW));

		Renderer {
			instance_mats: instance_mats
		}
	}

	pub fn render(&self, model: &Model) {
		model.draw(gl::TRIANGLES);
	}

	pub fn render_instanced(&self, model: &Model, transforms: &[Mat4]) {
		let sz = transforms.len() * mem::size_of::<Mat4>();

		GL!(BindBuffer(gl::ARRAY_BUFFER, self.instance_mats));
		GL!(BufferSubData(gl::ARRAY_BUFFER, 0, sz as _, transforms.as_ptr() as *const _));
		
		model.draw_instanced(gl::TRIANGLES, transforms.len() as i32);

		GL!(BindBuffer(gl::ARRAY_BUFFER, 0));
	}
}