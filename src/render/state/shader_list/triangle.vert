#version 300 es

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out vec4 VertColor;

void main() {
    gl_Position = vec4(Position, 1.0);
    VertColor = vec4(Color, 1.0);
}

