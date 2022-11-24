uniform sampler2D cur_tex;

in vec2 multi_tex_coord;
in float alpha;

out vec4 frag_color;

void main() {
	vec4 color = texture(cur_tex, multi_tex_coord);
	frag_color = vec4(color.rgb, color.a * alpha);
}
