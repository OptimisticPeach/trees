#version 320 es
uniform mat4x4 projection;
uniform mat4x4 model;
uniform mat4x4 view;

in vec4 color;
in vec4 pos;

out vec4 v_Color;

void main() {
    gl_Position = projection * view * model * pos;
    v_Color = color;
}
