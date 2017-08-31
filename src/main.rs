#![feature(slice_patterns)]

extern crate sdl2;
extern crate gl;

mod vecmath;
mod renderer;
mod logic;
mod primitives;

mod game;
use game::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

fn main() {
	let sdl = sdl2::init().unwrap();
	let video = sdl.video().unwrap();
	let mut time = sdl.timer().unwrap();

	let gl_attr = video.gl_attr();
	
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_flags().debug().set();
	gl_attr.set_context_version(3, 3);
	// gl_attr.set_multisample_buffers(1);
	// gl_attr.set_multisample_samples(8);

	let window = video.window("Test", 800, 600)
		.opengl()
		.build()
		.unwrap();

	let glc = window.gl_create_context().unwrap();
	window.gl_make_current(&glc).unwrap();

	gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

	let mut event_pump = sdl.event_pump().unwrap();

	let sz = window.size();
	let w = sz.0 as f32;
	let h = sz.1 as f32;

	let mut game = Game::new();
	game.on_init(w, h);

	let timeStep = 1.0f32 / 60.0;
	let mut startTime = 0f32;
	let mut accum = 0f32;
	let mut button_down = false;
	let mut mouse_button = MouseButton::Left;

	'running: loop {
		let current = time.ticks() as f32 / 1000.0;
		let delta = current - startTime;
		startTime = current;
		accum += delta;

		while accum >= timeStep {
			accum -= timeStep;

			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} => {
						break 'running
					},
					Event::KeyDown { keycode, .. } => {
						match keycode {
							Some(k) => { game.on_key_press(k); },
							None => {}
						}
					},
					Event::MouseButtonDown { mouse_btn, x, y, .. } => {
						mouse_button = mouse_btn;
						game.on_mouse_click(mouse_btn, x as f32, y as f32);
						button_down = true;
					},
					Event::MouseButtonUp { mouse_btn, .. } => {
						game.on_mouse_release(mouse_btn);
						button_down = false;
					},
					Event::MouseMotion { x, y, .. } => {
						game.on_mouse_move(x as f32, y as f32);
						if button_down {
							game.on_mouse_drag(mouse_button, x as f32, y as f32);
						}
					}
					_ => {}
				}
			}

			game.on_update(timeStep);
		}

		game.on_render(w, h);
		
		window.gl_swap_window();
	}

}
