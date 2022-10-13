#version 300 es

uniform mat4 matMVP;
uniform vec4 inColor;

layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 tex;
layout (location = 2) in float lval;

out vec2 multiTexCoord;
out vec4 color;

void main() {
	gl_Position = matMVP * pos;
	multiTexCoord = tex;
	color = vec4(inColor.rgb * vec3(lval, lval, lval), inColor.a);
}
