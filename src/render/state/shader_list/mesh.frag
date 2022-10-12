#version 300 es
precision mediump float;

in vec4 multiTexCoord;
in vec4 color;

out vec4 fragColor;

void main() {
    fragColor = color;
}