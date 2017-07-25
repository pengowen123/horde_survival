#version 150 core

in mat4 v_ModelView;
in mat4 v_Model;
in vec2 v_Uv;
in vec3 v_Normal;

in vec3 v_EyePos;
in vec3 v_FragPos;

in vec3 v_AmbientColor;
in float v_AmbientStrength;

in vec3 v_LightPos;
in vec3 v_LightColor;
in float v_LightStrength;
in float v_SpecularStrength;
in float v_SpecularFocus;

out vec4 Target0;

uniform sampler2D t_Color;

void main() {
	vec4 objectColor = texture(t_Color, v_Uv);

	// Calculate ambient light
	vec3 ambient = v_AmbientStrength * v_AmbientColor;

	// Normalize some values
	vec3 normal = normalize(v_Normal);
	vec3 lightDir = normalize(v_LightPos - v_FragPos);
	
	// Calculate diffuse light
	float diff = max(dot(normal, lightDir), 0.0);
	vec3 diffuse = diff * v_LightColor * v_LightStrength;

	// Calculate some values related to view direction
	vec3 viewDir = normalize(v_EyePos - v_FragPos);
	vec3 reflectDir = reflect(-lightDir, normal);

	// Calculate specular light
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), v_SpecularFocus);
	vec3 specular = v_SpecularStrength * spec * v_LightColor;

	// Calculate combined light
	vec3 light = ambient + diffuse + specular;

	vec4 color = vec4(light, 1.0) * objectColor;

	Target0 = color;
}

// vim: ft=glsl
