shader_type canvas_item;
render_mode unshaded;

uniform sampler2D viewport;
uniform vec2 resolution;
uniform bool enabled;
varying vec2 pixel_size;
const vec2 offset_uv = vec2(0.125, 0.375);

vec2 deriv_x(vec2 pos, vec4 frag, vec2 pixel) {
	vec2 offset = vec2(pixel.x, 0.0);
	vec2 pos_plus = pos + offset;
	vec2 pos_minus = pos - offset;
	int coord = int(frag.x) / 2;
	bool even = int(coord * 2) == int(frag.x);
	return even ? (pos_plus - pos) : (pos - pos_minus);
}

vec2 deriv_y(vec2 pos, vec4 frag, vec2 pixel) {
	vec2 offset = vec2(0.0, pixel.y);
	vec2 pos_plus = pos + offset;
	vec2 pos_minus = pos - offset;
	int coord = int(frag.y) / 2;
	bool even = int(coord * 2) == int(frag.y);
	return even ? (pos_plus - pos) : (pos - pos_minus);
}

vec4 ssaa(vec2 base_uv, vec4 frag, vec2 pixel) {
	vec2 dx = deriv_x(base_uv, frag, pixel);
	vec2 dy = deriv_y(base_uv, frag, pixel);
	vec2 uv = vec2(0.0);
	vec4 color = vec4(0.0);
	uv = base_uv + offset_uv.x * dx + offset_uv.y * dy;
	color += textureLod(viewport, uv, 0.0);
	uv = base_uv - offset_uv.x * dx - offset_uv.y * dy;
	color += textureLod(viewport, uv, 0.0);
	uv = base_uv + offset_uv.y * dx - offset_uv.x * dy;
	color += textureLod(viewport, uv, 0.0);
	uv = base_uv - offset_uv.y * dx + offset_uv.x * dy;
	color += textureLod(viewport, uv, 0.0);
	color *= 0.25;
	return color;
}

void vertex() {
	pixel_size = 1.0 / resolution;
}

void fragment() {
	COLOR = enabled ? ssaa(UV, FRAGCOORD, pixel_size) : textureLod(viewport, UV, 0.0);
}
