#version 300 es

precision highp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor;

uniform mat4 mvp;
smooth out vec4 fragmentColor;

void main(void){
    gl_Position = mvp*vec4(position, 1.0);
    fragmentColor = vertexColor;
}
