extern crate sdl2;
extern crate gl;
use std::mem;
use sdl2::mouse::MouseButton;
use std::path::Path;

use primitives;
use vecmath::*;
use renderer::*;
use logic::*;

pub struct Game {
	shader: Shader,
	cursor_tex: Texture,
	textures: [Texture; 12],
	model: Model,
	dmap: Map,
	proj: Mat4,
	view: Mat4,
	camera: Mat4,
	ax: f32,
	ay: f32,
	cursor_x: i32,
	cursor_y: i32,
	cam_pos: Vec3,
	mouse_pos: Vec2,
	mouse_prev_pos: Vec2
}

impl Drop for Game {
	fn drop(&mut self) {
		self.cursor_tex.free();
		for i in 0..self.textures.len() {
			self.textures[i].free();
		}
		self.model.free();
	}
}

impl Game {
	pub fn new() -> Game {
		let vs = include_str!("default.vs");
		let fs = include_str!("default.fs");
		
		let mut shd = Shader::new();
		shd.add_shader(vs, gl::VERTEX_SHADER);
		shd.add_shader(fs, gl::FRAGMENT_SHADER);
		shd.link();

		Game {
			shader: shd,
			cursor_tex: Texture::new(Path::new("res/cursor.png")),
			textures: [
				Texture::new(Path::new("res/road_s.png")), //0
				Texture::new(Path::new("res/road_s_h.png")), //1
				Texture::new(Path::new("res/road_c00.png")), //2
				Texture::new(Path::new("res/road_c10.png")), //3
				Texture::new(Path::new("res/road_c11.png")), //4
				Texture::new(Path::new("res/road_c01.png")), //5
				Texture::new(Path::new("res/road_t_l.png")), //6
				Texture::new(Path::new("res/road_t_u.png")), //7
				Texture::new(Path::new("res/road_t_r.png")), //8
				Texture::new(Path::new("res/road_t_d.png")), //9
				Texture::new(Path::new("res/road_cross.png")), //10
				Texture::new(Path::new("res/grass.png")) //11
			],
			model: primitives::make_plane(),
			dmap: Map::new(16, 16),
			proj: Mat4::identity(),
			view: Mat4::identity(),
			camera: Mat4::identity(),
			ax: 32.264f32.to_radians(),
			ay: -45.0f32.to_radians(),
			cursor_x: 0,
			cursor_y: 0,
			cam_pos: Vec3::new(0.0, 0.0, 0.0),
			mouse_pos: Vec2::new(0.0, 0.0),
			mouse_prev_pos: Vec2::new(0.0, 0.0)
		}
	}

	pub fn on_init(&mut self, w: f32, h: f32) {
		GL!(Enable(gl::DEPTH_TEST));
		GL!(Enable(gl::CULL_FACE));
		GL!(Enable(gl::BLEND));
		GL!(BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
		GL!(FrontFace(gl::CCW));

		let aspect = w / h;
		let scale = 4f32;

		self.proj = Mat4::ortho(-scale * aspect, scale * aspect, scale, -scale, -scale*10.0, scale*10.0);
		self.view = Mat4::rotation_x(self.ax) * Mat4::rotation_y(self.ay);
	}

	pub fn on_mouse_click(&mut self, button: MouseButton, x: f32, y: f32) {
		self.mouse_prev_pos.x = x;
		self.mouse_prev_pos.y = y;
		match button {
			MouseButton::Left => {
				if self.dmap.get_bit(self.cursor_x, self.cursor_y) == 0 {
					self.dmap.set_bit(self.cursor_x, self.cursor_y, 1);
					self.dmap.solve();
				}
			},
			MouseButton::Right => {
				if self.dmap.get_bit(self.cursor_x, self.cursor_y) == 1 {
					self.dmap.set_bit(self.cursor_x, self.cursor_y, 0);
					self.dmap.set(self.cursor_x, self.cursor_y, 11);
					self.dmap.solve();
				}
			},
			_ => { }
		}
	}

	pub fn on_mouse_release(&self, button: MouseButton) {

	}

	pub fn on_mouse_move(&mut self, x: f32, y: f32) {
		self.mouse_pos.x = x;
		self.mouse_pos.y = y;
	}

	pub fn on_mouse_drag(&mut self, button: MouseButton, x: f32, y: f32) {
		match button {
			MouseButton::Middle => {
				let dx = (x - self.mouse_prev_pos.x) * 0.01;
				let dy = (y - self.mouse_prev_pos.y) * 0.01;

				self.cam_pos.x += (self.ax.cos() * dx) + (self.ax.sin() * dy);
				self.cam_pos.z += (self.ax.sin() * dx) - (self.ax.cos() * dy);
				self.camera = Mat4::translation(self.cam_pos);

				self.mouse_prev_pos.x = x;
				self.mouse_prev_pos.y = y;
			},
			_ => {
				self.on_mouse_click(button, x, y);
			}
		}
	}

	pub fn on_update(&mut self, dt: f32) {

	}

	pub fn on_render(&mut self, w: f32, h: f32) {
		let viewmat = self.view.clone() * self.camera.clone();

		GL!(ClearColor(0.1_f32, 0.08_f32, 0.2_f32, 1.0_f32));
		GL!(Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

		self.shader.bind();
		self.shader.get("projection").unwrap().set(self.proj.clone());
		self.shader.get("view").unwrap().set(viewmat.clone());
		match self.shader.get("texture0") {
			Some(uniform) => { uniform.set(0); },
			None => {}
		}
		self.shader.get("disableTexture").unwrap().set(0);
		self.shader.get("color").unwrap().set(Vec4::new(1.0, 1.0, 1.0, 1.0));

		let mut last_tile = -1;
		for y in 0..self.dmap.height() {
			for x in 0..self.dmap.width() {
				let tile = self.dmap.get(x, y);
				if tile != last_tile {
					last_tile = tile;
					self.textures[tile as usize].bind(0);
				}
				self.shader.get("model").unwrap().set(Mat4::translation(Vec3::new(x as f32, 0.0, y as f32)));
				self.model.draw(gl::TRIANGLES);
			}
		}

		let ipv = (self.proj.clone() * viewmat.clone()).inverted();

		// Calculate cursor pos in world space
		let mut z = 0.0f32;
		GL!(ReadPixels(
			self.mouse_pos.x as i32,
			self.mouse_pos.y as i32,
			1, 1,
			gl::DEPTH_COMPONENT,
			gl::FLOAT,
			mem::transmute(&mut z)
		));
		
		let mut cur_pos = self.mouse_pos.extend(1.0-z).unproject(Vec4::new(0.0, 0.0, w, h), h, ipv);
		cur_pos.x = cur_pos.x.floor();
		cur_pos.z = cur_pos.z.floor();
		cur_pos.y = 0.0;

		// For the Map interaction
		self.cursor_x = cur_pos.x as i32;
		self.cursor_y = cur_pos.z as i32;

		self.cursor_tex.bind(0);
		self.shader.get("color").unwrap().set(Vec4::new(0.0, 0.3, 0.8, 1.0));
		self.shader.get("model").unwrap().set(Mat4::translation(cur_pos));

		GL!(Disable(gl::DEPTH_TEST));
		self.model.draw(gl::TRIANGLES);
		GL!(Enable(gl::DEPTH_TEST));

		self.shader.unbind();
	}
}