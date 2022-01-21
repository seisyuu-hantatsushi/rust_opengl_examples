#version 300 es

precision highp float;

smooth in vec4 lightIntensity;
out vec4 outputColor;

void main(void){
    outputColor = lightIntensity;
}
