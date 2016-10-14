#version 120

uniform sampler2D t_Color;

varying vec2 v_TexCoord;

void main() {
    vec4 tex = texture2D(t_Color, v_TexCoord);
    float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
    gl_FragColor = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
}
