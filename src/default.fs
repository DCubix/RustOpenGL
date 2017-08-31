#version 330
out vec4 fragColor;

in DATA {
	vec3 position;
	vec3 normal;
	vec2 uv;
} fs_in;

const vec3 lightDir = vec3(-1.0, -1.0, -1.0);
const vec3 ambient = vec3(0.3, 0.3, 0.5);

uniform vec4 color = vec4(1.0);
uniform bool disableTexture = false;
uniform sampler2D texture0;

float lind(float d, float n, float f) {
	return (2.0 * n) / (f + n - d * (f - n));
}

void main() {
	vec3 N = normalize(fs_in.normal);
	float nl = min(1.0, max(dot(N, -lightDir), 0.0));
	vec3 diff = vec3(nl) + ambient;

	vec4 col = vec4(1.0);
	if (!disableTexture) { col = texture(texture0, fs_in.uv); }

	// float d = gl_FragCoord.z * 100.0;
	fragColor = vec4(diff * color.rgb, 1.0) * col;
	// fragColor = vec4(vec3(d), 1.0);
}