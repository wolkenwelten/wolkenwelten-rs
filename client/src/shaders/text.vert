uniform mat4 mat_mvp;

in vec2 pos;
in vec2 tex;
in vec4 color;

out vec2 multi_tex_coord;
out vec4 front_color;

void main(){
	gl_Position     = mat_mvp * vec4(pos, 1.0, 1.0);
	multi_tex_coord = tex * 0.0078125; // 1/128
	front_color     = color;
}
