extern crate sdl2;
extern crate gl;

mod vecmath;
mod renderer;
mod primitives;
use renderer::*;

use vecmath::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
	let sdl = sdl2::init().unwrap();
	let video = sdl.video().unwrap();

	let gl_attr = video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_flags().debug().set();
	gl_attr.set_context_version(3, 3);

	let window = video.window("Test", 800, 600)
		.opengl()
		.build()
		.unwrap();

	let glc = window.gl_create_context().unwrap();
	window.gl_make_current(&glc).unwrap();

	gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

	let mut event_pump = sdl.event_pump().unwrap();

	let vs = "
		#version 330\n
		layout (location = 0) in vec3 v_pos;\n
		layout (location = 1) in vec3 v_nrm;\n
		layout (location = 2) in vec2 v_uv;\n
		out DATA {\n
			vec3 position;\n
			vec3 normal;\n
			vec2 uv;\n
		} vs_out;\n
		uniform mat4 projection;\n
		uniform mat4 model;\n
		uniform mat4 view;\n
		void main() {\n
			vec4 pos = model * vec4(v_pos, 1.0);
			gl_Position = projection * view * pos;\n
			mat3 nmat = mat3(transpose(inverse(model)));\n
			vs_out.position = pos.xyz;\n
			vs_out.normal = nmat * v_nrm;\n
			vs_out.uv = v_uv;\n
		}
	";
	let fs = "
		#version 330\n
		out vec4 fragColor;\n
		in DATA {\n
			vec3 position;\n
			vec3 normal;\n
			vec2 uv;\n
		} fs_in;\n
		const vec3 lightDir = vec3(-1.0, -1.0, 1.0);\n
		const vec3 ambient = vec3(0.12, 0.12, 0.2);\n
		void main() {\n
			vec3 N = normalize(fs_in.normal);\n

			float nl = max(dot(N, -lightDir), 0.0);\n
			vec3 diff = vec3(nl * 0.75) + ambient;\n
			fragColor = vec4(diff, 1.0);\n
		}\n
	";
	let mut shd = Shader::new();
	shd.add_shader(vs, gl::VERTEX_SHADER);
	shd.add_shader(fs, gl::FRAGMENT_SHADER);
	shd.link();

	let mut model = primitives::make_cube();

	let mut ren = Renderer::new();
	
	let sz = window.size();
	let w = sz.0 as f32;
	let h = sz.1 as f32;
	let aspect = w / h;
	let scale = 2f32;

	let d = (1.0_f32 / 3.0_f32).sqrt();
	let proj = Mat4::ortho(-scale * aspect, scale * aspect, scale, -scale, -scale, scale);
	// let proj = Mat4::perspective(60.0f32.to_radians(), aspect, 0.01f32, 1000f32);
	let view = Mat4::rotation_x(32.264f32.to_radians()) * Mat4::rotation_y(-45f32.to_radians());
	// let view = Mat4::translation(Vec3::new(-2.0, -1.0, -3.0));

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running
				},
				_ => {}
			}
		}

		GL!(ClearColor(0.1_f32, 0.08_f32, 0.2_f32, 1.0_f32));
		GL!(Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

		ren.submit(&model);

		shd.bind();
		shd.get("projection").unwrap().set(proj.clone());
		shd.get("view").unwrap().set(view.clone());
		shd.get("model").unwrap().set(Mat4::identity());

		ren.render();

		shd.unbind();
		
		window.gl_swap_window();
	}
	model.free();
}
