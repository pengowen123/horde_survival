#version 150

uniform sampler2D t_Color;

in vec2 v_TexCoord;
out vec4 f_Output;

void main() {
	vec4 tex = texture(t_Color, v_TexCoord);
	f_Output = tex;
}