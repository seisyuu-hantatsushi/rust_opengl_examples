#version 300 es

precision highp float;

smooth in vec4 fragmentColor;
out vec4 outputColor;

void main(void){
    outputColor = fragmentColor;
    //outputColor = vec4(1.0, 0.0, 0.0, 1.0);
}
