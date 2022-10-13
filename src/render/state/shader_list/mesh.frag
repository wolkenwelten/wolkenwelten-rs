#version 300 es
precision mediump float;
uniform sampler2D curTex;

in vec4 color;
in vec2 multiTexCoord;

out vec4 fragColor;

void main() {
	fragColor = texture2D(curTex, multiTexCoord) * color;
}