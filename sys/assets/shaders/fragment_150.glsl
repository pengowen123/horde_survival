#version 150 core

struct Material {
	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
	float shininess;
};

struct Light {
	vec3 position;

	vec3 ambient;
	vec3 diffuse;
	vec3 specular;
};

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

uniform Material u_Material;
uniform Light u_Light;

void main() {
	vec4 objectColor = texture(t_Color, v_Uv);

	// Ambient
	vec3 ambient = u_Light.ambient * u_Material.ambient;

	// Diffuse
	vec3 normal = normalize(v_Normal);
	vec3 u_LightDir = normalize(u_Light.position - v_FragPos);
	float diff = max(dot(normal, u_LightDir), 0.0);
	vec3 diffuse = u_Light.diffuse * (diff * u_Material.diffuse);

	// Specular
	vec3 viewDir = normalize(vec3(u_EyePos) - v_FragPos);
	vec3 reflectDir = reflect(-u_LightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material.shininess);
	vec3 specular = u_Light.specular * (spec * u_Material.specular);

	// Combined
	vec3 u_Light = ambient + diffuse + specular;
	vec4 color = vec4(u_Light, 1.0) * objectColor;
	Target0 = color;
}

// vim: ft=glsl
