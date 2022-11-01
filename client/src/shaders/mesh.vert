uniform mat4 mat_mvp;
uniform vec4 in_color;

in vec3 pos;
in vec2 tex;
in float lightness;

out vec2 multi_tex_coord;
out vec4 color;

void main() {
	gl_Position = mat_mvp * vec4(pos, 1.0);
	multi_tex_coord = tex;
	color = vec4(in_color.rgb * vec3(lightness, lightness, lightness), in_color.a);
}
