extern crate rand;
extern crate num;
use std::cmp::{ min, max };
use self::rand::{ thread_rng, Rng };

pub struct Map {
	bits: Vec<u8>,
	map: Vec<i32>,
	width: i32,
	height: i32
}

impl Map {
	pub fn new(width: i32, height: i32) -> Map {
		let mut map = Vec::new();
		let mut bits = Vec::new();
		map.resize((width * height) as usize, 11i32);
		bits.resize((width * height) as usize, 0u8);
		Map { map, bits, width, height }
	}

	fn lim_x(&self, x: i32) -> i32 {
		min(self.width-1, max(x, 0))
	}

	fn lim_y(&self, y: i32) -> i32 {
		min(self.height-1, max(y, 0))
	}

	pub fn solve(&mut self) {
		for y in 0..self.height {
			for x in 0..self.width {
				if self.get_bit(x, y) == 0 { continue; }
				self.solve_one(x, y);
			}
		}
	}

	pub fn valid(&self, x: i32, y: i32) -> bool {
		x >= 0 && x < self.width && y >= 0 && y < self.height
	}

	fn get_neighbors(&self, x: i32, y: i32) -> Vec<(i32, i32, u8)> {
		let mut vec = Vec::new();
		if self.valid(x-1, y) {
			vec.push((x-1, y, self.get_bit(x-1, y)));
		}
		if self.valid(x, y-1) {
			vec.push((x, y-1, self.get_bit(x, y-1)));
		}
		if self.valid(x+1, y) {
			vec.push((x+1, y, self.get_bit(x+1, y)));
		}
		if self.valid(x, y+1) {
			vec.push((x, y+1, self.get_bit(x, y+1)));
		}
		vec
	}

	pub fn get_tile(&self, x: i32, y: i32) -> i32 {
		let s = [
			self.get_bit(x, y-1),
			self.get_bit(x+1, y),
			self.get_bit(x, y+1),
			self.get_bit(x-1, y)
		];

		let bf = s[3] * 8 + s[2] * 4 + s[1] * 2 + s[0];
		match bf {
			0 => { 0 },
			1 => { 13 },
			2 => { 14 },
			3 => { 3 },
			4 => { 15 },
			5 => { 0 },
			6 => { 4 },
			7 => { 8 },
			8 => { 12 },
			9 => { 2 },
			10 => { 1 },
			11 => { 7 },
			12 => { 5 },
			13 => { 6 },
			14 => { 9 },
			15 => { 10 },
			_ => { 11 }
		}
	}

	pub fn solve_one(&mut self, x: i32, y: i32) -> i32 {
		let solved = self.get_tile(x, y);
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

	pub fn get(&self, x: i32, y: i32) -> i32 {
		self.map[(self.lim_x(x) + self.lim_y(y) * self.width) as usize]
	}

	pub fn set(&mut self, x: i32, y: i32, val: i32) {
		let lx = self.lim_x(x);
		let ly = self.lim_x(y);
		self.map[(lx + ly * self.width) as usize] = val;
	}

	pub fn get_random_road_point(&self) -> (i32, i32) {
		let mut rng = thread_rng();
		let mut x: i32 = rng.gen_range(0, self.width-1);
		let mut y: i32 = rng.gen_range(0, self.height-1);

		let mut bit = self.get_bit(x, y);
		while bit == 0 {
			x = rng.gen_range(0, self.width-1);
			y = rng.gen_range(0, self.height-1);
			bit = self.get_bit(x, y);
			// println!("CHECKED: X={}, Y={} => {}", x, y, bit);
		}
		(x, y)
	}

	pub fn find_path(&self, sx: i32, sy: i32, ex: i32, ey: i32) -> Vec<(i32, i32)> {
		let mut seen: Vec<bool> = Vec::new();
		seen.resize((self.width * self.height) as usize, false);
		
		let mut c = 0usize;
		let mut path = Vec::new();
		path.push((sx, sy));

		loop {
			if c >= seen.len() {
				return path;
			}

			let (cx, cy) = path[path.len().wrapping_sub(1)];
			seen[(cx + cy * self.width) as usize] = true;
			
			c += 1;

			let neighs = self.get_neighbors(cx, cy);
			let mut i = 0;
			for &(x, y, val) in neighs.iter() {
				if x == ex && y == ey {
					path.push((x, y));
					return path;
				}
				if !seen[(x + y * self.width) as usize] && val == 1 {
					path.push((x, y));
					break;
				} else {
					i += 1;
				}
			}

			if i == neighs.len() {
				path.pop();
			}
		}

	}

	pub fn width(&self) -> i32 { self.width }
	pub fn height(&self) -> i32 { self.height }

}