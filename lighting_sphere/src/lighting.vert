#version 300 es

precision highp float;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 vertexColor;

uniform mat4 model; //model transform matrix. to move, to scale, to rotation object
uniform mat4 view; //view transform matrix. to move camera position
uniform mat4 projection; //projection transform matrix. to project to screen
uniform mat3 normalMatrix;

uniform vec4 lightPosition;
uniform vec3 Ld; // strength of Light;

out vec4 lightIntensity;

smooth out vec4 fragmentColor;
//const vec3 lightDir = normalize(vec3(8.0, 4.0, 2.0));
//const vec3 lightColor = vec3(1.0,1.0,1.0);

void main(void){
    mat4 modelview = view*model;
    vec3 Kd = vec3(vertexColor.x, vertexColor.y, vertexColor.z);
    vec3 tnorm = normalize(normalMatrix * normal);
    vec4 eyeCoords = view*model*vec4(position, 1.0);
    vec3 s = normalize(vec3(lightPosition - eyeCoords));

    mat4 mvp = projection*modelview;

    //calc reflection
    lightIntensity = vec4(Ld*Kd*max(dot(s,tnorm),0.0),vertexColor.w);

    gl_Position = mvp*vec4(position, 1.0);

}
