#version 150 core

in vec2 v_Uv;
in vec3 v_Normal;
in vec3 v_FragPos;

out vec4 Target0;

uniform sampler2D t_Color;

uniform u_Locals {
	mat4 u_MVP;
	mat4 u_Model;
	vec3 u_EyePos;
};

uniform u_Material {
	vec3 u_Material_ambient;
	float padding_0;
	vec3 u_Material_diffuse;
	float padding_1;
	vec3 u_Material_specular;
	float padding_2;
	float u_Material_shininess;
};

uniform u_Light {
	vec3 u_Light_position;
	float padding_3;
	vec3 u_Light_ambient;
	float padding_4;
	vec3 u_Light_diffuse;
	float padding_5;
	vec3 u_Light_specular;
};

void main() {
	vec4 objectColor = texture(t_Color, v_Uv);

	// Ambient
	vec3 ambient = u_Light_ambient * u_Material_ambient;

	// Diffuse
	vec3 normal = normalize(v_Normal);
	vec3 u_LightDir = normalize(u_Light_position - v_FragPos);
	float diff = max(dot(normal, u_LightDir), 0.0);
	vec3 diffuse = u_Light_diffuse * (diff * u_Material_diffuse);

	// Specular
	vec3 viewDir = normalize(vec3(u_EyePos) - v_FragPos);
	vec3 reflectDir = reflect(-u_LightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material_shininess);
	vec3 specular = u_Light_specular * (spec * u_Material_specular);

	// Combined
	vec3 u_Light = ambient + diffuse + specular;
	vec4 color = vec4(u_Light, 1.0) * objectColor;
	Target0 = color;
}

// vim: ft=glsl
