#version 300 es
precision mediump float;

uniform mat4 matMVP;

layout (location = 0) in vec4 pos;
layout (location = 1) in vec2 tex;
layout (location = 2) in vec4 color;

out vec2 multiTexCoord;
out vec4 frontColor;

void main(){
    //gl_Position   = matMVP * pos;
    gl_Position   = pos;
    multiTexCoord = tex * 0.0078125; // 1/128
    frontColor    = color;
}
