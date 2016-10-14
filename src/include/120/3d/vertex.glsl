#version 120

uniform mat4 u_Transform;

attribute vec4 a_Pos;
attribute vec2 a_TexCoord;
varying vec2 v_TexCoord;

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * a_Pos;
    gl_ClipDistance[0] = 1.0;
}
