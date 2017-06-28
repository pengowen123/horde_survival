#version 150 core

in vec4 a_Pos;
in vec2 a_Uv;

uniform Locals {
	mat4 u_Transform;
	vec4 u_Pos;
};

out vec2 v_Uv;

void main() {
	v_Uv = a_Uv;
	gl_Position = u_Transform * (a_Pos + u_Pos);
	gl_ClipDistance[0] = 1.0;
}
