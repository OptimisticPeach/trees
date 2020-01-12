#version 320 es
uniform mat4x4 projection;
uniform mat4x4 model;
uniform mat4x4 view;

in vec4 color;
in vec4 pos;
in vec3 normal;
in vec2 uv;

out vec4 v_Color;
out vec3 v_Pos;
out vec3 v_Normal;
out vec2 v_UV;

void main() {
    gl_Position = projection * view * model * pos;
    v_Color = color;
    v_Normal = normalize((view * model * vec4(normal, 0.0)).xyz);
    v_Pos = vec3(view * model * pos);
    v_UV = uv;
}
