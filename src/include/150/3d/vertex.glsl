#version 150 core

uniform Locals {
	mat4 u_Transform;
};

in vec4 a_Pos;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * a_Pos;
    gl_ClipDistance[0] = 1.0;
}
