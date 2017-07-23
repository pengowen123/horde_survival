#version 150 core

in vec4 a_Pos;
in vec2 a_Uv;
in vec3 a_Normal;

uniform Locals {
	// Model view projection
	mat4 u_MVP;
	// Model view
	mat4 u_MV;
	// Model
	mat4 u_M;
};

out vec2 v_Uv;
out vec3 v_Normal;

void main() {
	v_Uv = a_Uv;
	v_Normal = a_Normal;

	gl_Position = u_MVP * a_Pos;

	gl_ClipDistance[0] = 1.0;
}

// vim: ft=glsl
