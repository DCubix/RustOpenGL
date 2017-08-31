use renderer::{ Model, VertexFormat, VertexAttribute };

pub fn make_plane() -> Model {
	let mut verts = [
		0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
		1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
		1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0,
		0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0,
	];

	let inds = [
		1, 0, 3, 3, 2, 1
	];

	let fmt = VertexFormat::new(&[
		VertexAttribute::new(3, false),
		VertexAttribute::new(3, false),
		VertexAttribute::new(2, false)
	]);

	Model::from(&verts, &inds, fmt)
}