#version 300 es

precision highp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 vertexColor;

uniform mat4 model; //model transform matrix. to move, to scale, to rotation object
uniform mat4 view; //view transform matrix. to move camera position
uniform mat4 projection; //projection transform matrix. to project to screen

smooth out vec4 fragmentColor;
const vec3 lightDir = normalize(vec3(8.0, 4.0, 2.0));
const vec3 lightColor = vec3(1.0,1.0,1.0);

void main(void){

    vec3 diffuseColor = vec3(dot(position, lightDir))*lightColor*vec3(vertexColor.x,vertexColor.y,vertexColor.z);
    mat4 mvp = projection*view*model;

    //calc reflection
    
    gl_Position = mvp*vec4(position, 1.0);
    fragmentColor = vec4(diffuseColor, vertexColor.w);
}
