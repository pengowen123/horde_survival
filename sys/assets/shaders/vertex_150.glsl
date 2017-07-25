#version 150 core

in vec3 a_Pos;
in vec2 a_Uv;
in vec3 a_Normal;

// Matrices
out mat4 v_ModelView;
out mat4 v_Model;

// Texture coordinate
out vec2 v_Uv;
out vec3 v_Normal;

out vec3 v_EyePos;
out vec3 v_FragPos;

out vec3 v_AmbientColor;
out float v_AmbientStrength;

out vec3 v_LightPos;
out vec3 v_LightColor;
out float v_LightStrength;
out float v_SpecularStrength;
out float v_SpecularFocus;

layout(std140) uniform Locals {
	// Model view projection
	mat4 u_MVP;
	// Model view
	mat4 u_ModelView;
	// Model
	mat4 u_Model;

	vec4 u_LightPos;
	vec4 u_LightColor;
	vec4 u_AmbientColor;
	vec4 u_EyePos;
	float u_LightStrength;
	float u_AmbientStrength;
	float u_SpecularStrength;
	float u_SpecularFocus;
};

void main() {
	v_ModelView = u_ModelView;
	v_Model = u_Model;
	v_Uv = a_Uv;
	v_Normal = mat3(transpose(inverse(u_Model))) * a_Normal;
	v_EyePos = vec3(u_Model * u_EyePos);
	v_FragPos = vec3(u_Model * vec4(a_Pos, 1.0));
	v_AmbientColor = vec3(u_AmbientColor);
	v_AmbientStrength = u_AmbientStrength;
	v_LightPos = vec3(u_LightPos);
	v_LightColor = vec3(u_LightColor);
	v_LightStrength = u_LightStrength;
	v_SpecularStrength = u_SpecularStrength;
	v_SpecularFocus = u_SpecularFocus;

	gl_Position = u_MVP * vec4(a_Pos, 1.0);

	gl_ClipDistance[0] = 1.0;
}

// vim: ft=glsl
