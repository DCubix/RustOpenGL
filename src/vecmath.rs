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

	pub fn unproject(&self, viewport: Vec4, wheight: f32, invpv: Mat4) -> Vec3 {
		let mut x = self.x;
		let mut y = self.y;
		x = x - viewport.x;
		y = wheight - y - 1.0;
		y = y - viewport.y;
		let sx = (2.0 * x) / viewport.z - 1.0;
		let sy = (2.0 * y) / viewport.w - 1.0;
		let sz = 2.0 * self.z - 1.0;
		(invpv * Vec4::new(sx, sy, sz, 1.0)).to_vec3()
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

impl Index<usize> for Mat4 {
	type Output = Vec4;
	fn index(&self, i: usize) -> &Vec4 {
		&self.rows[i]
	}
}

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

	pub fn from_rows(r0: Vec4, r1: Vec4, r2: Vec4, r3: Vec4) -> Mat4 {
		Mat4 { rows: [ r0, r1, r2, r3 ] }
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

	pub fn inverted(&self) -> Mat4 {
		let m = self.clone();

		let coef00 = m[2][2] * m[3][3] - m[3][2] * m[2][3];
		let coef02 = m[1][2] * m[3][3] - m[3][2] * m[1][3];
		let coef03 = m[1][2] * m[2][3] - m[2][2] * m[1][3];
		let coef04 = m[2][1] * m[3][3] - m[3][1] * m[2][3];
		let coef06 = m[1][1] * m[3][3] - m[3][1] * m[1][3];
		let coef07 = m[1][1] * m[2][3] - m[2][1] * m[1][3];
		let coef08 = m[2][1] * m[3][2] - m[3][1] * m[2][2];
		let coef10 = m[1][1] * m[3][2] - m[3][1] * m[1][2];
		let coef11 = m[1][1] * m[2][2] - m[2][1] * m[1][2];
		let coef12 = m[2][0] * m[3][3] - m[3][0] * m[2][3];
		let coef14 = m[1][0] * m[3][3] - m[3][0] * m[1][3];
		let coef15 = m[1][0] * m[2][3] - m[2][0] * m[1][3];
		let coef16 = m[2][0] * m[3][2] - m[3][0] * m[2][2];
		let coef18 = m[1][0] * m[3][2] - m[3][0] * m[1][2];
		let coef19 = m[1][0] * m[2][2] - m[2][0] * m[1][2];
		let coef20 = m[2][0] * m[3][1] - m[3][0] * m[2][1];
		let coef22 = m[1][0] * m[3][1] - m[3][0] * m[1][1];
		let coef23 = m[1][0] * m[2][1] - m[2][0] * m[1][1];

		let fac0 = Vec4::new(coef00, coef00, coef02, coef03);
		let fac1 = Vec4::new(coef04, coef04, coef06, coef07);
		let fac2 = Vec4::new(coef08, coef08, coef10, coef11);
		let fac3 = Vec4::new(coef12, coef12, coef14, coef15);
		let fac4 = Vec4::new(coef16, coef16, coef18, coef19);
		let fac5 = Vec4::new(coef20, coef20, coef22, coef23);

		let vec0 = Vec4::new(m[1][0], m[0][0], m[0][0], m[0][0]);
		let vec1 = Vec4::new(m[1][1], m[0][1], m[0][1], m[0][1]);
		let vec2 = Vec4::new(m[1][2], m[0][2], m[0][2], m[0][2]);
		let vec3 = Vec4::new(m[1][3], m[0][3], m[0][3], m[0][3]);

		let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
		let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
		let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
		let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

		let signA = Vec4::new(1.0, -1.0, 1.0, -1.0);
		let signB = Vec4::new(-1.0, 1.0, -1.0, 1.0);
	 	let inverse = Mat4::from_rows(inv0 * signA, inv1 * signB, inv2 * signA, inv3 * signB);

		let row0 = Vec4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);

		let dot0 = m[0] * row0;
		let dot1 = (dot0[0] + dot0[1]) + (dot0[2] + dot0[3]);

		let oneOverDeterminant = 1.0f32 / dot1;

		inverse * oneOverDeterminant
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

	pub fn det(&self) -> f32 {
		  self.rows[0].x * self.rows[1].y * self.rows[2].z * self.rows[3].w - self.rows[0].x * self.rows[1].y * self.rows[2].w * self.rows[3].z + self.rows[0].x * self.rows[1].z * self.rows[2].w * self.rows[3].y - self.rows[0].x * self.rows[1].z * self.rows[2].y * self.rows[3].w
		+ self.rows[0].x * self.rows[1].w * self.rows[2].y * self.rows[3].z - self.rows[0].x * self.rows[1].w * self.rows[2].z * self.rows[3].y - self.rows[0].y * self.rows[1].z * self.rows[2].w * self.rows[3].x + self.rows[0].y * self.rows[1].z * self.rows[2].x * self.rows[3].w
		- self.rows[0].y * self.rows[1].w * self.rows[2].x * self.rows[3].z + self.rows[0].y * self.rows[1].w * self.rows[2].z * self.rows[3].x - self.rows[0].y * self.rows[1].x * self.rows[2].z * self.rows[3].w + self.rows[0].y * self.rows[1].x * self.rows[2].w * self.rows[3].z
		+ self.rows[0].z * self.rows[1].w * self.rows[2].x * self.rows[3].y - self.rows[0].z * self.rows[1].w * self.rows[2].y * self.rows[3].x + self.rows[0].z * self.rows[1].x * self.rows[2].y * self.rows[3].w - self.rows[0].z * self.rows[1].x * self.rows[2].w * self.rows[3].y
		+ self.rows[0].z * self.rows[1].y * self.rows[2].w * self.rows[3].x - self.rows[0].z * self.rows[1].y * self.rows[2].x * self.rows[3].w - self.rows[0].w * self.rows[1].x * self.rows[2].y * self.rows[3].z + self.rows[0].w * self.rows[1].x * self.rows[2].z * self.rows[3].y
		- self.rows[0].w * self.rows[1].y * self.rows[2].z * self.rows[3].x + self.rows[0].w * self.rows[1].y * self.rows[2].x * self.rows[3].z - self.rows[0].w * self.rows[1].z * self.rows[2].x * self.rows[3].y + self.rows[0].w * self.rows[1].z * self.rows[2].y * self.rows[3].x
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

impl Mul<f32> for Mat4 {
	type Output = Mat4;
	fn mul(self, rhs: f32) -> Mat4 {
		Mat4::from_rows(
			self.rows[0] * rhs,
			self.rows[1] * rhs,
			self.rows[2] * rhs,
			self.rows[3] * rhs
		)
	}
}