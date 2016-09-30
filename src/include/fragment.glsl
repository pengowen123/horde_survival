#version 150
// TODO: Add texture support to render 2d text

uniform sampler2D t_Color;

in vec2 v_TexCoord;
out vec4 f_Color;

void main() {
	vec4 tex = texture(t_Color, v_TexCoord);
	f_Color = tex;
}
