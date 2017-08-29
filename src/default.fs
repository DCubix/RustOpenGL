#version 330
out vec4 fragColor;

in DATA {
	vec3 position;
	vec3 normal;
	vec2 uv;
} fs_in;

const vec3 lightDir = vec3(-1.0, -1.0, 1.0);
const vec3 ambient = vec3(0.12, 0.12, 0.2);

uniform vec4 color = vec4(1.0);
uniform bool disableTexture = false;
uniform sampler2D texture0;

void main() {
	vec3 N = normalize(fs_in.normal);
	float nl = max(dot(N, -lightDir), 0.0);
	vec3 diff = (vec3(nl) * color.rgb) + ambient;

	vec4 col = vec4(1.0);
	if (!disableTexture) { col = texture(texture0, fs_in.uv); }

	fragColor = vec4(diff, color.a) * col;
}