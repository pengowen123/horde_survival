#version 150 core

// A directional light
struct DirLight {
	vec4 direction;

	vec4 ambient;
	vec4 diffuse;
	vec4 specular;
};

vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir);

// A point light
struct PointLight {
	vec4 position;

	vec4 ambient;
	vec4 diffuse;
	vec4 specular;

	float constant;
	float linear;
	float quadratic;
};

vec4 CalcPointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 fragPos);

// A spotlight
struct SpotLight {
	vec4 position;
	vec4 direction;

	vec4 ambient;
	vec4 diffuse;
	vec4 specular;

	float cutOff;
	float outerCutOff;
};

vec4 CalcSpotLight(SpotLight light, vec3 normal, vec3 viewDir, vec3 fragPos);

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

	float u_Light_constant;
	float u_Light_linear;
	float u_Light_quadratic;
};

void main() {
	// Sample textures
	vec4 objectColor = texture(t_Color, v_Uv);
	// Properties
	vec3 norm = normalize(v_Normal);
	vec3 viewDir = normalize(vec3(u_EyePos) - v_FragPos);

	SpotLight light;

	light.position = vec4(0.0, 0.0, 10.0, 1.0);
	light.direction = vec4(0.0, 0.0, -1.0, 0.0);
	light.ambient = u_Light_ambient;
	light.diffuse = u_Light_diffuse;
	light.specular = u_Light_specular;
	light.cutOff = cos(0.2);
	light.outerCutOff = cos(0.3);

	vec4 result = CalcSpotLight(light, norm, viewDir, v_FragPos);

	vec4 color = result * objectColor;
	Target0 = color;
}

vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir) {
	vec3 lightDir = normalize(vec3(-light.direction));

	// Diffuse
	float diff = max(dot(normal, lightDir), 0.0);

	// Specular
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material_shininess);

	// Apply lighting maps and light properties
	vec4 ambient = light.ambient * texture(t_Diffuse, v_Uv);
	vec4 diffuse = light.diffuse * (diff * texture(t_Diffuse, v_Uv));
	vec4 specular = light.specular * (spec * texture(t_Specular, v_Uv));

	return (ambient + diffuse + specular);
}

vec4 CalcPointLight(PointLight light, vec3 normal, vec3 viewDir, vec3 fragPos) {
	vec3 lightDir = normalize(vec3(light.position) - fragPos);

	// Diffuse
	float diff = max(dot(normal, lightDir), 0.0);

	// Specular
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material_shininess);

	// Attenuation
	float dist = length(vec3(light.position) - fragPos);
	float attenuation  = 1.0 / (
			light.constant +
			light.linear * dist +
			light.quadratic * (dist * dist));

	// Apply lighting maps and light properties
	vec4 ambient = light.ambient * texture(t_Diffuse, v_Uv);
	vec4 diffuse = light.diffuse * (diff * texture(t_Diffuse, v_Uv));
	vec4 specular = light.specular * (spec * texture(t_Specular, v_Uv));

	// Apply attenuation
	ambient *= attenuation;
	diffuse *= attenuation;
	specular *= attenuation;

	return (ambient + diffuse + specular);
}

vec4 CalcSpotLight(SpotLight light, vec3 normal, vec3 viewDir, vec3 fragPos) {
	vec3 lightDir = normalize(vec3(light.position) - fragPos);

	vec4 result;

	// Diffuse
	float diff = max(dot(normal, lightDir), 0.0);

	// Specular
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material_shininess);

	// Apply lighting maps and light properties
	vec4 ambient = light.ambient * texture(t_Diffuse, v_Uv);
	vec4 diffuse = light.diffuse * (diff * texture(t_Diffuse, v_Uv));
	vec4 specular = light.specular * (spec * texture(t_Specular, v_Uv));

	// Calculate intensity of the spotlight based on the angle
	float theta = dot(lightDir, normalize(vec3(-light.direction)));
	float epsilon = light.cutOff - light.outerCutOff;
	float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

	// TODO: maybe remove ambient scaling (at least for testing that the spotlight works on a real
	//		 map)
	ambient *= intensity;
	diffuse *= intensity;
	specular *= intensity;

	return (ambient + diffuse + specular);
}

// vim: ft=glsl
