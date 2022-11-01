uniform sampler2D cur_tex;

in vec4 color;
in vec2 multi_tex_coord;

out vec4 frag_color;

void main() {
	frag_color = texture(cur_tex, multi_tex_coord) * color;
}