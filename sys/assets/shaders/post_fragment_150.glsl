#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Screen;

out vec4 Target0;

void main() {
	vec4 color = texture(t_Screen, v_Uv);
	Target0 = color;
}

// vim: ft=glsl
