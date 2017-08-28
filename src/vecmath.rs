use std::ops::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2 { pub x: f32, pub y: f32 }

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 {
		Vec2 { x: x, y: y }
	}

	pub fn zero() -> Vec2 { Vec2::new(0.0, 0.0) }

	pub fn from_slice(s: &[f32]) -> Vec2 {
		assert!(s.len() >= 2);
		Vec2::new(s[0], s[1])
	}

	pub fn extend(&self, z: f32) -> Vec3 {
		Vec3::new(self.x, self.y, z)
	}

	pub fn dot(&self, other: Vec2) -> f32 {
		self.x * other.x + self.y * other.y
	}

	pub fn perp_dot(&self, other: Vec2) -> f32 {
		self.x * other.y - self.y * other.x
	}

	pub fn length(self) -> f32 { 
		self.dot(self).sqrt()
	}

	pub fn normalized(&self) -> Vec2 {
		let len = self.length();
		Vec2 { x: self.x / len, y: self.y / len }
	}

}

impl Add<Vec2> for Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Vec2) -> Vec2 {
		Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}

impl Sub<Vec2> for Vec2 {
	type Output = Vec2;
	fn sub(self, rhs: Vec2) -> Vec2 {
		Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
	}
}

impl Mul<Vec2> for Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: Vec2) -> Vec2 {
		Vec2 { x: self.x * rhs.x, y: self.y * rhs.y }
	}
}

impl Mul<f32> for Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: f32) -> Vec2 {
		Vec2 { x: self.x * rhs, y: self.y * rhs }
	}
}

impl Neg for Vec2 {
	type Output = Vec2;
	fn neg(self) -> Vec2 {
		Vec2 { x: -self.x, y: -self.y }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 { x: x, y: y, z: z }
	}

	pub fn zero() -> Vec3 { Vec3::new(0.0, 0.0, 0.0) }

	pub fn from_slice(s: &[f32]) -> Vec3 {
		assert!(s.len() >= 3);
		Vec3::new(s[0], s[1], s[2])
	}

	pub fn extend(&self, w: f32) -> Vec4 {
		Vec4::new(self.x, self.y, self.z, w)
	}

	pub fn dot(&self, other: Vec3) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn cross(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * self.x
		}
	}

	pub fn length(&self) -> f32 {
		self.dot(self.clone()).sqrt()
	}

	pub fn normalized(&self) -> Vec3 {
		let len = self.length();
		Vec3 { x: self.x / len, y: self.y / len, z: self.z / len }
	}

}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
	}
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: Vec3) -> Vec3 {
		Vec3 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: f32) -> Vec3 {
		Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
	}
}

impl Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Vec3 {
		Vec3 { x: -self.x, y: -self.y, z: -self.z }
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec4 { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

impl Index<usize> for Vec4 {
	type Output = f32;
	fn index(&self, i: usize) -> &f32 {
		match i {
			0 => { &self.x },
			1 => { &self.y },
			2 => { &self.z },
			_ => { &self.w }
		}
	}
}

impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
		Vec4 { x: x, y: y, z: z, w: w }
	}

	pub fn to_vec3(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}

	pub fn from_slice(s: &[f32]) -> Vec4 {
		assert!(s.len() >= 4);
		Vec4::new(s[0], s[1], s[2], s[3])
	}

	pub fn dot(&self, other: Vec4) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
	}

	pub fn length(&self) -> f32 {
		self.dot(self.clone()).sqrt()
	}

	pub fn normalized(&self) -> Vec4 {
		let len = self.length();
		Vec4 { x: self.x / len, y: self.y / len, z: self.z / len, w: self.w / len }
	}
}

impl Add<Vec4> for Vec4 {
	type Output = Vec4;
	fn add(self, rhs: Vec4) -> Vec4 {
		Vec4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
	}
}

impl Sub<Vec4> for Vec4 {
	type Output = Vec4;
	fn sub(self, rhs: Vec4) -> Vec4 {
		Vec4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
	}
}

impl Mul<Vec4> for Vec4 {
	type Output = Vec4;
	fn mul(self, rhs: Vec4) -> Vec4 {
		Vec4 { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
	}
}

impl Mul<f32> for Vec4 {
	type Output = Vec4;
	fn mul(self, rhs: f32) -> Vec4 {
		Vec4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
	}
}

impl Neg for Vec4 {
	type Output = Vec4;
	fn neg(self) -> Vec4 {
		Vec4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
	}
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Mat4 { rows: [Vec4; 4] }

impl Mat4 {
	pub fn new(m: &[f32; 16]) -> Mat4 {
		Mat4 {
			rows: [
				Vec4::from_slice(&m[0..4]),
				Vec4::from_slice(&m[4..8]),
				Vec4::from_slice(&m[8..12]),
				Vec4::from_slice(&m[12..16])
			]
		}
	}

	pub fn identity() -> Mat4 {
		Mat4::uniform_scaling(1.0)
	}
	
	pub fn translation(t: Vec3) -> Mat4 {
		Mat4::new(&[
			1.0, 0.0, 0.0, t.x,
			0.0, 1.0, 0.0, t.y,
			0.0, 0.0, 1.0, t.z,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn rotation_x(a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		Mat4::new(&[
			1.0, 0.0, 0.0, 0.0,
			0.0,   c,  -s, 0.0,
			0.0,   s,   c, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn rotation_y(a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		Mat4::new(&[
			  c, 0.0,  -s, 0.0,
			0.0, 1.0, 0.0, 0.0,
			  s, 0.0,   c, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn rotation_z(a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		Mat4::new(&[
			  c,  -s, 0.0, 0.0,
			  s,   c, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn axis_angle(axis: Vec3, a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		let invC = 1.0 - c;
		Mat4::new(&[
			invC * axis.x * axis.x + c,
			invC * axis.x * axis.y + s * axis.z,
			invC * axis.x * axis.z - s * axis.y,
			0.0,
			invC * axis.x * axis.y - s * axis.z,
			invC * axis.y * axis.y + c,
			invC * axis.y * axis.z + s * axis.x,
			0.0,
			invC * axis.x * axis.z + s * axis.y,
			invC * axis.y * axis.z - s * axis.x,
			invC * axis.z * axis.z + c,
			0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn scaling(s: Vec3) -> Mat4 {
		Mat4::new(&[
			s.x, 0.0, 0.0, 0.0,
			0.0, s.y, 0.0, 0.0,
			0.0, 0.0, s.z, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn uniform_scaling(s: f32) -> Mat4 { Mat4::scaling(Vec3::new(s, s, s)) }

	pub fn transpose(&self) -> Mat4 {
		let a = self.rows[0];
		let b = self.rows[1];
		let c = self.rows[2];
		let d = self.rows[3];
		Mat4::new(&[
			a.x, b.x, c.x, d.x,
			a.y, b.y, c.y, d.y,
			a.z, b.z, c.z, d.z,
			a.w, b.w, c.w, d.w,
		])
	}

	pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Mat4 {
		let w = r - l;
		let h = t - b;
		let d = f - n;

		Mat4::new(&[
			2.0 / w,	 0.0,	   0.0, -(r + l) / w,
				0.0, 2.0 / h,	   0.0, -(t + b) / h,
				0.0,	 0.0, -2.0 / d, -(f + n) / d,
				0.0,	 0.0,	   0.0,			 1.0,
		])
	}

	pub fn perspective(fov: f32, asp: f32, n: f32, f: f32) -> Mat4 {
		let cot = 1.0 / (fov / 2.0).tan();
		let d = n - f;

		Mat4::new(&[
			cot / asp, 0.0, 		0.0,			   0.0,
				  0.0, cot,			0.0,			   0.0,
				  0.0, 0.0,	(f + n) / d, (2.0 * f * n) / d,
				  0.0, 0.0,		   -1.0,			   0.0
		])
	}

	pub fn look_at(eye: Vec3, at: Vec3, up: Vec3) -> Mat4 {
		let z = (at - eye).normalized();
		let x = z.cross(up).normalized();
		let y = z.cross(x);

		Mat4::new(&[
			 x.x,  x.y,  x.z, -eye.dot(x),
			 y.x,  y.y,  y.z, -eye.dot(y),
			 z.x,  z.y,  z.z, -eye.dot(z),
			 0.0,  0.0,  0.0, 1.0
		])
	}

	pub fn as_ptr(&self) -> *const f32 {
		&self.rows[0].x
	}

}

impl Mul<Mat4> for Mat4 {
	type Output = Mat4;
	fn mul(self, rhs: Mat4) -> Mat4 {
		let mut d = [0.0f32; 16];
		let ot = rhs.transpose();

		for j in 0..4 {
			for i in 0..4 {
				d[i + j * 4] = self.rows[j].dot(ot.rows[i]);
			}
		}

		Mat4::new(&d)
	}
}

impl Mul<Vec4> for Mat4 {
	type Output = Vec4;
	fn mul(self, rhs: Vec4) -> Vec4 {
		Vec4::new(
			self.rows[0].dot(rhs),
			self.rows[1].dot(rhs),
			self.rows[2].dot(rhs),
			self.rows[3].dot(rhs),
		)
	}
}

impl Mul<Vec3> for Mat4 {
	type Output = Vec3;
	fn mul(self, rhs: Vec3) -> Vec3 {
		let v = rhs.extend(1.0);
		Vec3::new(
			self.rows[0].dot(v),
			self.rows[1].dot(v),
			self.rows[2].dot(v)
		)
	}
}