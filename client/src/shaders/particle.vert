uniform mat4 mat_mvp;
uniform float size_mul;

in vec4 pos;
in vec4 color;

out vec4 frontColor;

void main(){
	gl_Position  = mat_mvp * vec4(pos.xyz,1.0);
	gl_PointSize = pos.w / (gl_Position.z) * size_mul;
	frontColor   = color;
}
