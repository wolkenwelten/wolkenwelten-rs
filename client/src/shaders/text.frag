uniform sampler2D cur_tex;

in vec2 multi_tex_coord;
in vec4 front_color;

out vec4 frag_color;

void main() {
	frag_color = front_color * texture(cur_tex, multi_tex_coord);
}
