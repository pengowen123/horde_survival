#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Screen;

out vec4 Target0;

const float offset = 1.0 / 300.0;

void main() {
	vec2 offsets[9] = vec2[](
		vec2(-offset,  offset),
		vec2( 0.0,	   offset),
		vec2( offset,  offset),
		vec2(-offset,  0.0),
		vec2( 0.0,	   0.0),
		vec2( offset,  0.0),
		vec2(-offset, -offset),
		vec2( 0.0,	  -offset),
		vec2( offset, -offset)
	);

	float kernel[9] = float[](
		1.0 / 16, 2.0 / 16, 1.0 / 16,
		2.0 / 16, 4.0 / 16, 2.0 / 16,
		1.0 / 16, 2.0 / 16, 1.0 / 16  
	);

	vec3 sampleTex[9];

	for (int i = 0; i < 9; i++) {
		sampleTex[i] = vec3(texture(t_Screen, v_Uv.xy + offsets[i]));
	}

	vec3 color = vec3(0.0);

	for (int i = 0; i < 9; i++) {
		color += sampleTex[i] * kernel[i];
	}

	Target0 = vec4(color, 1.0);
}


// vim: ft=glsl
