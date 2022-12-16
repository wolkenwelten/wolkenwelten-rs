uniform sampler2DArray cur_tex;
uniform float color_alpha;

in vec3 tex_coord;
in float light_value;

out vec4 frag_color;

void main() {
/* Very simple shader, we look up the currents pixel color according to
 | the texCoord passed to us, and then multiply in order to darken the
 | color according to the current lightness level.  The alpha value is
 | stored as a uniform because we only fadeIn entire chunks just after
 | they have been generated so their sudden appearance is less jarring.
 */
	vec3 light_color = vec3(light_value, light_value, light_value);
	vec4 color = vec4(texture(cur_tex, tex_coord).rgb * light_color, color_alpha);
	frag_color = color;
}
