extern crate sdl2;
extern crate gl;
use std::mem;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::f32::consts::PI;

use primitives;
use vecmath::*;
use renderer::*;
use logic::*;

#[derive(Debug, Clone)]
struct Car {
	pub pos: Vec2,
	pub dir: Vec2,
	speed: f32,
	waypoints: Vec<Vec2>,
	current_way: usize,
	stopped: bool
}

impl Car {
	pub fn new(map: &Map) -> Car {
		let (sx, sy) = map.get_random_road_point();
		let ep = map.get_random_road_point();

		let waypoints = map.find_path(sx, sy, ep.0, ep.1)
						   .into_iter()
						   .map(|i| Vec2::new(i.0 as f32 + 0.5, i.1 as f32 + 0.5))
						   .collect();
		Car {
			pos: Vec2::new(sx as f32 + 0.5, sy as f32 + 0.5),
			dir: Vec2::new(0.0, 0.0),
			speed: 1.5,
			waypoints: waypoints,
			current_way: 0,
			stopped: false
		}
	}

	fn refresh(&mut self, map: &Map) {
		let mut lx = 0;
		let mut ly = 0;
		if self.waypoints.len() > 0 {
			let last_w = self.current_waypoint();
			lx = (last_w.x - 0.5) as i32;
			ly = (last_w.y - 0.5) as i32;
		} else {
			let (x, y) = map.get_random_road_point();
			lx = x; ly = y;
		}

		let ep = map.get_random_road_point();
		// println!("{:?} -> {:?}", (lx, ly), ep);

		let path = map.find_path(lx, ly, ep.0, ep.1);

		self.waypoints = path.into_iter()
							.map(|i| Vec2::new(i.0 as f32 + 0.5, i.1 as f32 + 0.5))
							.collect();
		self.current_way = 0;
	}

	pub fn current_waypoint(&self) -> Vec2 {
		self.waypoints[self.current_way]
	}

	pub fn last_waypoint(&self) -> Vec2 {
		self.waypoints[self.waypoints.len().wrapping_sub(1)]
	}

	pub fn render(&self, shader: &mut Shader, car_tex: &Texture, cursor_tex: &Texture, car: &Model, model: &Model) {
		car_tex.bind(0);
		shader.get("color").unwrap().set(Vec4::new(1.0, 1.0, 1.0, 1.0));
		let rot = Mat4::rotation_y(self.dir.y.atan2(self.dir.x) + PI/2.0);
		shader.get("model").unwrap().set(
			Mat4::translation(Vec3::new(self.pos.x, 0.0, self.pos.y)) * rot
		);
		car.draw(gl::TRIANGLES);

		// Draw first
		cursor_tex.bind(0);
		shader.get("color").unwrap().set(Vec4::new(1.0, 0.5, 0.0, 1.0));
		let f = self.waypoints[0];
		shader.get("model").unwrap().set(
			Mat4::translation(Vec3::new(f.x-0.5, 0.0, f.y-0.5))
		);
		GL!(Disable(gl::DEPTH_TEST));
		model.draw(gl::TRIANGLES);
		GL!(Enable(gl::DEPTH_TEST));

		if self.waypoints.len() > 2 {
			shader.get("color").unwrap().set(Vec4::new(1.0, 1.0, 1.0, 1.0));
			for i in 1..self.waypoints.len()-1 {
				let w = self.waypoints[i];
				shader.get("model").unwrap().set(
					Mat4::translation(Vec3::new(w.x-0.5, 0.0, w.y-0.5))
				);
				GL!(Disable(gl::DEPTH_TEST));
				model.draw(gl::TRIANGLES);
				GL!(Enable(gl::DEPTH_TEST));
			}
		}

		// Draw last
		shader.get("color").unwrap().set(Vec4::new(0.0, 1.0, 0.0, 1.0));
		let l = self.last_waypoint();
		shader.get("model").unwrap().set(
			Mat4::translation(Vec3::new(l.x-0.5, 0.0, l.y-0.5))
		);
		GL!(Disable(gl::DEPTH_TEST));
		model.draw(gl::TRIANGLES);
		GL!(Enable(gl::DEPTH_TEST));
	}

	pub fn update(&mut self, dt: f32, map: &Map) {
		if self.waypoints.len() == 0 { self.stopped = true; }
		if self.stopped { return; }

		let w = self.current_waypoint();
		let v = w - self.pos;

		self.dir = v.normalized();
		self.pos = self.pos + (self.dir * self.speed) * dt;

		let dist = v.length();
		if dist <= 0.25 {
			if self.current_way+1 < self.waypoints.len() {
				self.current_way += 1;
			} else {
				self.refresh(map);
			}
		}
	}
}

pub struct Game {
	pub shader: Shader,
	pub cursor_tex: Texture,
	textures: [Texture; 16],
	pub model: Model,
	house: Model,
	pub car: Model,
	house_tex: Texture,
	pub car_tex: Texture,
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
	mouse_prev_pos: Vec2,
	cars: Vec<Car>
}

impl Drop for Game {
	fn drop(&mut self) {
		self.cursor_tex.free();
		for i in 0..self.textures.len() {
			self.textures[i].free();
		}
		self.model.free();
		self.house.free();
		self.house_tex.free();
		self.car.free();
		self.car_tex.free();
	}
}

impl Game {
	pub fn new() -> Game {
		let vs = include_str!("default.vs");
		let fs = include_str!("default.fs");
		
		let shd = Shader::new();
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
				Texture::new(Path::new("res/grass.png")), //11
				Texture::new(Path::new("res/road_e_l.png")), //12
				Texture::new(Path::new("res/road_e_u.png")), //13
				Texture::new(Path::new("res/road_e_r.png")), //14
				Texture::new(Path::new("res/road_e_d.png")), //15
			],
			model: primitives::make_plane(),
			house: Model::from_file(Path::new("res/house.obj"), true).unwrap(),
			house_tex: Texture::new(Path::new("res/house_tex.png")),
			car: Model::from_file(Path::new("res/car.obj"), true).unwrap(),
			car_tex: Texture::new(Path::new("res/car_tex.png")),
			dmap: Map::new(24, 24),
			proj: Mat4::identity(),
			view: Mat4::identity(),
			camera: Mat4::identity(),
			ax: -32.264f32.to_radians(),
			ay: -45.0f32.to_radians(),
			cursor_x: 0,
			cursor_y: 0,
			cam_pos: Vec3::new(0.0, 0.0, 0.0),
			mouse_pos: Vec2::new(0.0, 0.0),
			mouse_prev_pos: Vec2::new(0.0, 0.0),
			cars: Vec::new()
		}
	}

	pub fn on_init(&mut self, w: f32, h: f32) {
		GL!(Enable(gl::DEPTH_TEST));
		GL!(Enable(gl::BLEND));
		GL!(BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
		GL!(FrontFace(gl::CCW));

		let aspect = w / h;
		let scale = 5f32;

		self.proj = Mat4::ortho(-scale * aspect, scale * aspect, scale, -scale, -scale*10.0, scale*10.0);
		// self.proj = Mat4::perspective(45f32.to_radians(), aspect, 0.01, 1000.0);
		self.view = Mat4::rotation_x(self.ax) * Mat4::rotation_y(self.ay);
		// self.view = Mat4::rotation_x((PI/2.0).to_radians());
		self.view = self.view.clone() * Mat4::scaling(Vec3::new(1.0, -1.0, 1.0));
		// self.view = Mat4::translation(Vec3::new(1.0, -0.25, -4.0));
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
				let d = Vec2::new(dx, dy).rotate(PI/4.0);

				self.cam_pos.x += d.x;
				self.cam_pos.z += d.y;
				self.camera = Mat4::translation(self.cam_pos);

				self.mouse_prev_pos.x = x;
				self.mouse_prev_pos.y = y;
			},
			_ => {
				self.on_mouse_click(button, x, y);
			}
		}
	}

	pub fn on_key_press(&mut self, key: Keycode) {
		match key {
			Keycode::Return => { self.cars.push(Car::new(&self.dmap)); },
			_ => {}
		}
	}

	pub fn on_update(&mut self, dt: f32) {
		for car in self.cars.iter_mut() {
			car.update(dt, &self.dmap);
		}
	}

	pub fn on_render(&mut self, w: f32, h: f32) {
		GL!(ClearColor(0.1_f32, 0.08_f32, 0.2_f32, 1.0_f32));
		GL!(Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

		let viewmat = self.view.clone() * self.camera.clone();

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

		// self.house_tex.bind(0);
		// self.shader.get("model").unwrap().set(Mat4::translation(Vec3::new(0.0, 0.0, 0.0)));
		// self.house.draw(gl::TRIANGLES);

		for car in self.cars.iter() {
			car.render(&mut self.shader, &self.car_tex, &self.cursor_tex, &self.car, &self.model);
		}

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
		
		let mut cur_pos = self.mouse_pos.extend(1.0 - z).unproject(
			Vec4::new(0.0, 0.0, w, h),
			viewmat.clone(),
			self.proj.clone()
		);
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