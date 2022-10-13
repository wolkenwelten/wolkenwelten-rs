#version 300 es

uniform mat4 matMVP;

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out vec4 VertColor;

void main() {
	gl_Position = matMVP * vec4(Position, 1.0);
	VertColor = vec4(Color, 1.0);
}
