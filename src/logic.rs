extern crate num;
use std::cmp::{ min, max };

pub struct Map {
	bits: Vec<u8>,
	map: Vec<i32>,
	width: i32,
	height: i32
}

impl Map {
	pub fn new(width: i32, height: i32) -> Map {
		let mut m = Vec::new();
		let mut b = Vec::new();
		m.resize((width * height) as usize, 11i32);
		b.resize((width * height) as usize, 0u8);
		Map {
			map: m,
			bits: b,
			width: width,
			height: height
		}
	}

	fn lim_x(&self, x: i32) -> i32 {
		min(max(x, 0), self.width-1)
	}

	fn lim_y(&self, y: i32) -> i32 {
		min(max(y, 0), self.height-1)
	}

	pub fn solve(&mut self) {
		for y in 0..self.height {
			for x in 0..self.width {
				if self.get_bit(x, y) == 0 { continue; }
				self.solve_one(x, y);
			}
		}
	}

	pub fn solve_one(&mut self, x: i32, y: i32) -> i32 {
		let s = [
			self.get_bit(x, y-1),
			self.get_bit(x+1, y),
			self.get_bit(x, y+1),
			self.get_bit(x-1, y)
		];

		let bf = s[3] * 8 + s[2] * 4 + s[1] * 2 + s[0];
		let solved = match bf {
			0 => { 0 },
			1 => { 0 },
			2 => { 1 },
			3 => { 3 },
			4 => { 0 },
			5 => { 0 },
			6 => { 4 },
			7 => { 8 },
			8 => { 1 },
			9 => { 2 },
			10 => { 1 },
			11 => { 7 },
			12 => { 5 },
			13 => { 6 },
			14 => { 9 },
			15 => { 10 },
			_ => { 11 }
		};
		self.set(x, y, solved);
		solved
	}

	pub fn get_bit(&self, x: i32, y: i32) -> u8 {
		self.bits[(self.lim_x(x) + self.lim_y(y) * self.width) as usize]
	}

	pub fn set_bit(&mut self, x: i32, y: i32, val: u8) {
		let lx = self.lim_x(x);
		let ly = self.lim_x(y);
		self.bits[(lx + ly * self.width) as usize] = val;
	}

	pub fn set_bit_mat(&mut self, x: i32, y: i32, val: &[u8], valw: i32, valh: i32) {
		for vy in 0..valh {
			for vx in 0..valw {
				let mx = x + vx;
				let my = y + vy;
				self.set_bit(mx, my, val[(vx + vy * valw) as usize]);
			}
		}
	}

	pub fn get(&self, x: i32, y: i32) -> i32 {
		self.map[(self.lim_x(x) + self.lim_y(y) * self.width) as usize]
	}

	pub fn set(&mut self, x: i32, y: i32, val: i32) {
		let lx = self.lim_x(x);
		let ly = self.lim_x(y);
		self.map[(lx + ly * self.width) as usize] = val;
	}

	pub fn width(&self) -> i32 { self.width }
	pub fn height(&self) -> i32 { self.height }

}