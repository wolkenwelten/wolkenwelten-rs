#version 300 es
precision mediump float;

in vec4 VertColor;
out vec4 FragColor;

void main() {
    FragColor = VertColor;
}