#version 150 core

in vec3 a_Pos;
in vec2 a_Uv;
in vec3 a_Normal;

// Texture coordinate
out vec2 v_Uv;
// Normal vector
out vec3 v_Normal;
// Position of the fragment
out vec3 v_FragPos;

uniform u_Locals {
	// MVP matrix
	mat4 u_MVP;
	// Model matrix
	mat4 u_Model;
	vec4 u_EyePos;
};

void main() {
	v_Uv = a_Uv;
	v_Normal = mat3(transpose(inverse(u_Model))) * a_Normal;
	v_FragPos = vec3(u_Model * vec4(a_Pos, 1.0));

	gl_Position = u_MVP * vec4(a_Pos, 1.0);
}

// vim: ft=glsl
