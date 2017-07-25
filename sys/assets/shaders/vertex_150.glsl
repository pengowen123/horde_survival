#version 150 core

in vec4 a_Pos;
in vec2 a_Uv;
in vec3 a_Normal;

out mat4 v_ModelView;
out vec2 v_Uv;
out vec3 v_Normal;
out vec4 v_AmbientColor;
out vec4 v_LightPos;
out vec4 v_LightColor;
out vec4 v_EyePos;
out vec4 v_FragPos;
out float v_LightStrength;
out float v_AmbientStrength;

layout(std140) uniform Locals {
	// Model view projection
	mat4 u_MVP;
	// Model view
	mat4 u_MV;
	// Model
	mat4 u_M;

	vec4 u_LightPos;
	vec4 u_LightColor;
	vec4 u_AmbientColor;
	vec4 u_EyePos;
	float u_LightStrength;
	float u_AmbientStrength;
};

void main() {
	v_ModelView = u_MV;
	v_Uv = a_Uv;
	v_Normal = a_Normal;
	v_AmbientColor = u_AmbientColor;
	v_AmbientStrength = u_AmbientStrength;
	v_LightPos = u_LightPos;
	v_LightColor = u_LightColor;
	v_LightStrength = u_LightStrength;
	v_EyePos = u_EyePos;
	v_FragPos = u_MV * a_Pos;

	gl_Position = u_MVP * a_Pos;

	gl_ClipDistance[0] = 1.0;
}

// vim: ft=glsl
