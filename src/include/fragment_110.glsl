#version 110

uniform sampler2D t_Color;

varying vec2 v_Uv;

void main() {
	vec4 tex = texture2D(t_Color, v_Uv);
	gl_FragColor = tex;
}
