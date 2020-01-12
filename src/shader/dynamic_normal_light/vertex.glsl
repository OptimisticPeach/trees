#version 320 es
uniform mat4x4 projection;
uniform mat4x4 model;
uniform mat4x4 view;
uniform mediump sampler3D perlin_source;
uniform mat4x4 texture_scaler;

in vec4 v_Color;
in vec4 v_Pos;
in float v_Opacity;

out PerVertex {
    vec4 g_Color;
    vec3 g_Pos;
    float f_Opacity;
} v_In;

void main() {
    vec3 perlin_pos = (texture_scaler * v_Pos).xyz;
    vec4 pos = v_Pos;
    pos.y += texture(perlin_source, perlin_pos).r - 0.5;
//    pos.xyz = perlin_pos;
    gl_Position = projection * view * model * pos;
    v_In.g_Color = v_Color;
    v_In.g_Pos = vec3(view * model * pos);
    v_In.f_Opacity = v_Opacity;
}
