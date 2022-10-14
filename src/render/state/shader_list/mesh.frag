uniform sampler2D curTex;

in vec4 color;
in vec2 multiTexCoord;

out vec4 fragColor;

void main() {
	fragColor = texture(curTex, multiTexCoord) * color;
}