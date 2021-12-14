#version 330
in vec3 position;
in vec4 color;

smooth out vec4 vertexColor;

void main(void){
    gl_Position = vec4(position, 1.0);
    vertexColor = color;
}
