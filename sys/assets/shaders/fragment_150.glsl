#version 150 core

in vec2 v_Uv;
in vec3 v_Normal;

uniform sampler2D t_Color;
uniform Locals {
	// Model view projection
	mat4 u_MVP;
	// Model view
	mat4 u_MV;
	// Model
	mat4 u_M;
};

out vec4 Target0;

void main() {
    vec4 tex = texture(t_Color, v_Uv);
    float blend = dot(v_Uv - vec2(0.5, 0.5), v_Uv - vec2(0.5, 0.5));
    Target0 = mix(tex, vec4(0.0, 0.0, 0.0, 0.0), blend * 1.0);
}

// vim: ft=glsl
