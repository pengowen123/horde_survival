#version 150 core

in vec2 v_Uv;
in vec3 v_Normal;
in vec3 v_FragPos;

out vec4 Target0;

uniform sampler2D t_Color;
uniform sampler2D t_Diffuse;
uniform sampler2D t_Specular;

uniform u_Locals {
	mat4 u_MVP;
	mat4 u_Model;
	vec4 u_EyePos;
};

uniform u_Material {
	float u_Material_shininess;
};

uniform u_Light {
	vec4 u_Light_position;
	vec4 u_Light_ambient;
	vec4 u_Light_diffuse;
	vec4 u_Light_specular;
};

void main() {
	vec4 objectColor = texture(t_Color, v_Uv);
	vec4 objectDiffuse = texture(t_Diffuse, v_Uv);
	vec4 objectSpecular = texture(t_Specular, v_Uv);

	// Ambient
	vec4 ambient = u_Light_ambient * objectDiffuse;

	// Diffuse
	vec3 normal = normalize(v_Normal);
	vec3 u_LightDir = normalize(vec3(u_Light_position) - v_FragPos);
	float diff = max(dot(normal, u_LightDir), 0.0);
	vec4 diffuse = u_Light_diffuse * (diff * objectDiffuse);

	// Specular
	vec3 viewDir = normalize(vec3(u_EyePos) - v_FragPos);
	vec3 reflectDir = reflect(-u_LightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material_shininess);
	vec4 specular = u_Light_specular * (spec * objectSpecular);

	// Combined
	vec4 u_Light = ambient + diffuse + specular;
	vec4 color = u_Light * objectColor;
	Target0 = color;
}

// vim: ft=glsl
