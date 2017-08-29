#version 330

layout (location = 0) in vec3 v_pos;
layout (location = 1) in vec3 v_nrm;
layout (location = 2) in vec2 v_uv;

out DATA {
	vec3 position;
	vec3 normal;
	vec2 uv;
} vs_out;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
	vec4 pos = model * vec4(v_pos, 1.0);
	gl_Position = projection * view * pos;

	mat3 nmat = mat3(transpose(inverse(model)));
	vs_out.position = pos.xyz;
	vs_out.normal = nmat * v_nrm;
	vs_out.uv = v_uv;
}