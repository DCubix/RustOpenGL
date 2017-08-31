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

	pub fn rotate(&self, a: f32) -> Vec2 {
		let (s, c) = a.sin_cos();
		Vec2::new((c * self.x) - (s * self.y), (s * self.x) + (c * self.y))
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
		let mut len = self.length();
		if len <= 0.0 { len = 1.0; }
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

	pub fn unproject(&self, viewport: Vec4, model_view: Mat4, projection: Mat4) -> Vec3 {
		let invpv = (projection * model_view).inverted();

		let w = viewport[2] - viewport[0];
		let h = viewport[3] - viewport[1];
		
		let x = (2.0 * (self.x - viewport.x) / w) - 1.0;
		let y = -((2.0 * (self.y - viewport.y) / h) - 1.0);
		let z = 2.0 * self.z - 1.0;

		let r_cast = invpv * Vec4::new(x, y, z, 1.0);
		Vec3::new(
			r_cast.x / r_cast.w,
			r_cast.y / r_cast.w,
			r_cast.z / r_cast.w
		)
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

impl IndexMut<usize> for Vec4 {
	fn index_mut(&mut self, i: usize) -> &mut f32 {
		match i {
			0 => { &mut self.x },
			1 => { &mut self.y },
			2 => { &mut self.z },
			_ => { &mut self.w }
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
#[derive(Debug, Copy, Clone)]
pub struct Mat4 { rows: [Vec4; 4] }

impl Index<usize> for Mat4 {
	type Output = Vec4;
	fn index(&self, i: usize) -> &Vec4 {
		&self.rows[i]
	}
}

impl IndexMut<usize> for Mat4 {
	fn index_mut(&mut self, i: usize) -> &mut Vec4 {
		&mut self.rows[i]
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
		let t = 1.0 - c;
		let ax = axis.normalized();
		let x = ax.x;
		let y = ax.y;
		let z = ax.z;
		Mat4::new(&[
			t * x * x + c, t * x * y - z * s, t * x * z + y * s, 0.0,
			t * x * y + z * s, t * y * y + c, t * y * z - x * s, 0.0,
			t * x * z - y * s, t * y * z + x * s, t * z * z + c, 0.0,
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
		let [a, b, c, d] = self.rows;
		Mat4::new(&[
			a.x, b.x, c.x, d.x,
			a.y, b.y, c.y, d.y,
			a.z, b.z, c.z, d.z,
			a.w, b.w, c.w, d.w,
		])
	}

	pub fn inverted(&self) -> Mat4 {
		//
		// Inversion by Cramer's rule.  Code taken from an Intel publication
		//
		let mut mat = self.clone();
		let mut tmp = [0.0f32; 12];
		let mut src = [0.0f32; 16];

		// Transpose
		for i in 0..4 {
			src[i + 0] = self[i][0];
			src[i + 4] = self[i][1];
			src[i + 8] = self[i][2];
			src[i + 12] = self[i][3];
		}

		// Calculate pairs for first 8 elements (cofactors)
		tmp[0] = src[10] * src[15];
		tmp[1] = src[11] * src[14];
		tmp[2] = src[9] * src[15];
		tmp[3] = src[11] * src[13];
		tmp[4] = src[9] * src[14];
		tmp[5] = src[10] * src[13];
		tmp[6] = src[8] * src[15];
		tmp[7] = src[11] * src[12];
		tmp[8] = src[8] * src[14];
		tmp[9] = src[10] * src[12];
		tmp[10] = src[8] * src[13];
		tmp[11] = src[9] * src[12];

		// Calculate first 8 elements (cofactors)
		mat[0][0] = tmp[0] * src[5] + tmp[3] * src[6] + tmp[4] * src[7];
		mat[0][0] -= tmp[1] * src[5] + tmp[2] * src[6] + tmp[5] * src[7];
		mat[0][1] = tmp[1] * src[4] + tmp[6] * src[6] + tmp[9] * src[7];
		mat[0][1] -= tmp[0] * src[4] + tmp[7] * src[6] + tmp[8] * src[7];
		mat[0][2] = tmp[2] * src[4] + tmp[7] * src[5] + tmp[10] * src[7];
		mat[0][2] -= tmp[3] * src[4] + tmp[6] * src[5] + tmp[11] * src[7];
		mat[0][3] = tmp[5] * src[4] + tmp[8] * src[5] + tmp[11] * src[6];
		mat[0][3] -= tmp[4] * src[4] + tmp[9] * src[5] + tmp[10] * src[6];
		mat[1][0] = tmp[1] * src[1] + tmp[2] * src[2] + tmp[5] * src[3];
		mat[1][0] -= tmp[0] * src[1] + tmp[3] * src[2] + tmp[4] * src[3];
		mat[1][1] = tmp[0] * src[0] + tmp[7] * src[2] + tmp[8] * src[3];
		mat[1][1] -= tmp[1] * src[0] + tmp[6] * src[2] + tmp[9] * src[3];
		mat[1][2] = tmp[3] * src[0] + tmp[6] * src[1] + tmp[11] * src[3];
		mat[1][2] -= tmp[2] * src[0] + tmp[7] * src[1] + tmp[10] * src[3];
		mat[1][3] = tmp[4] * src[0] + tmp[9] * src[1] + tmp[10] * src[2];
		mat[1][3] -= tmp[5] * src[0] + tmp[8] * src[1] + tmp[11] * src[2];

		// Calculate pairs for second 8 elements (cofactors)
		tmp[0] = src[2] * src[7];
		tmp[1] = src[3] * src[6];
		tmp[2] = src[1] * src[7];
		tmp[3] = src[3] * src[5];
		tmp[4] = src[1] * src[6];
		tmp[5] = src[2] * src[5];
		tmp[6] = src[0] * src[7];
		tmp[7] = src[3] * src[4];
		tmp[8] = src[0] * src[6];
		tmp[9] = src[2] * src[4];
		tmp[10] = src[0] * src[5];
		tmp[11] = src[1] * src[4];

		// Calculate second 8 elements (cofactors)
		mat[2][0] = tmp[0] * src[13] + tmp[3] * src[14] + tmp[4] * src[15];
		mat[2][0] -= tmp[1] * src[13] + tmp[2] * src[14] + tmp[5] * src[15];
		mat[2][1] = tmp[1] * src[12] + tmp[6] * src[14] + tmp[9] * src[15];
		mat[2][1] -= tmp[0] * src[12] + tmp[7] * src[14] + tmp[8] * src[15];
		mat[2][2] = tmp[2] * src[12] + tmp[7] * src[13] + tmp[10] * src[15];
		mat[2][2] -= tmp[3] * src[12] + tmp[6] * src[13] + tmp[11] * src[15];
		mat[2][3] = tmp[5] * src[12] + tmp[8] * src[13] + tmp[11] * src[14];
		mat[2][3] -= tmp[4] * src[12] + tmp[9] * src[13] + tmp[10] * src[14];
		mat[3][0] = tmp[2] * src[10] + tmp[5] * src[11] + tmp[1] * src[9];
		mat[3][0] -= tmp[4] * src[11] + tmp[0] * src[9] + tmp[3] * src[10];
		mat[3][1] = tmp[8] * src[11] + tmp[0] * src[8] + tmp[7] * src[10];
		mat[3][1] -= tmp[6] * src[10] + tmp[9] * src[11] + tmp[1] * src[8];
		mat[3][2] = tmp[6] * src[9] + tmp[11] * src[11] + tmp[3] * src[8];
		mat[3][2] -= tmp[10] * src[11] + tmp[2] * src[8] + tmp[7] * src[9];
		mat[3][3] = tmp[10] * src[10] + tmp[4] * src[8] + tmp[9] * src[9];
		mat[3][3] -= tmp[8] * src[9] + tmp[11] * src[10] + tmp[5] * src[8];

		// Calculate determinant
		let det = 1.0f32 / (src[0] * mat[0][0] + src[1] * mat[0][1] + src[2] * mat[0][2] + src[3] * mat[0][3]);
		for i in 0..4 {
			for j in 0..4 {
				mat[i][j] = mat[i][j] * det;
			}
		}
		mat
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
				  0.0, 0.0,		    -1.0,			   0.0
		])
	}

	pub fn look_at(eye: Vec3, at: Vec3, up: Vec3) -> Mat4 {
		let z = (eye - at).normalized();
		let x = up.cross(z).normalized();
		let y = z.cross(x);

		let R = Mat4::new(&[
			x.x, x.y, -x.z, 0.0,
			y.x, y.y, -y.z, 0.0,
			z.x, z.y, -z.z, 0.0,
			0.0, 0.0, 0.0, 1.0
		]);

		Mat4::translation(-eye) * R
	}

	pub fn as_ptr(&self) -> *const f32 {
		&self.rows[0][0]
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