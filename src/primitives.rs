use renderer::{ Model, VertexAttribute };

pub fn make_plane() -> Model {
	let mut verts = [
		0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
		1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
		1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
		0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0,
	];

	let inds = [
		0, 1, 2, 2, 3, 0
	];

	let fmt = [
		VertexAttribute::new(3, false),
		VertexAttribute::new(3, false),
		VertexAttribute::new(2, false)
	];

	Model::from(&verts, &inds, &fmt)
}