use renderer::{ Model, VertexAttribute };
use vecmath::Vec3;
use std::f32::consts::PI;

// The format of the models generated
// by this library is:
// X, Y, Z, NX, NY, NZ, U, V

pub enum Axis {
	X, Y, Z
}

fn gen_unit_plane(axis: Axis, off: f32) -> [f32; 32] {
	match axis {
		Axis::X => {
			[
				off, 0.0,  0.0,  1.0, 0.0, 0.0, 0.0, 0.0,
				off, 0.0, -1.0,  1.0, 0.0, 0.0, 1.0, 0.0,
				off, 1.0, -1.0,  1.0, 0.0, 0.0, 1.0, 1.0,
				off, 1.0,  0.0,  1.0, 0.0, 0.0, 0.0, 1.0
			]
		},
		Axis::Y => {
			[
				0.0, off,  0.0,  0.0, 1.0, 0.0, 0.0, 0.0,
				1.0, off,  0.0,  0.0, 1.0, 0.0, 1.0, 0.0,
				1.0, off, -1.0,  0.0, 1.0, 0.0, 1.0, 1.0,
				0.0, off, -1.0,  0.0, 1.0, 0.0, 0.0, 1.0,
			]
		},
		Axis::Z => {
			[
				0.0, 0.0, off,  0.0, 0.0, 1.0, 0.0, 0.0,
				1.0, 0.0, off,  0.0, 0.0, 1.0, 1.0, 0.0,
				1.0, 1.0, off,  0.0, 0.0, 1.0, 1.0, 1.0,
				0.0, 1.0, off,  0.0, 0.0, 1.0, 0.0, 1.0,
			]
		}
	}
}

pub fn make_cube() -> Model {
	let mut verts = Vec::new();
	verts.extend(gen_unit_plane(Axis::X, 0.0).to_vec());
	verts.extend(gen_unit_plane(Axis::X, 1.0).to_vec());
	verts.extend(gen_unit_plane(Axis::Y, 0.0).to_vec());
	verts.extend(gen_unit_plane(Axis::Y, 1.0).to_vec());
	verts.extend(gen_unit_plane(Axis::Z, 1.0).to_vec());
	verts.extend(gen_unit_plane(Axis::Z, -1.0).to_vec());

	let inds = [
		1, 0, 3, 3, 2, 1, // RIGHT
		4, 5, 6, 6, 7, 4, // LEFT
		9, 8, 11, 11, 10, 9, // TOP
		12, 13, 14, 14, 15, 12, // BOTTOM
		16, 17, 18, 18, 19, 16, // BACK
		21, 20, 23, 23, 22, 21, // FRONT
	];

	let fmt = [
		VertexAttribute::new(3, false),
		VertexAttribute::new(3, false),
		VertexAttribute::new(2, false)
	];

	Model::from(verts.as_slice(), &inds, &fmt)
}