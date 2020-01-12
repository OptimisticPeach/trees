#version 320 es
precision mediump float;
uniform vec3 light;
uniform vec3 eye;
uniform highp mat4x4 view;
uniform highp mat4x4 model;
uniform vec4 light_colour;
uniform sampler2D texture_source;

in vec4 v_Color;
in vec3 v_Pos;
in vec3 v_Normal;
in vec2 v_UV;
out vec4 outColor;

void main() {
    // https://www.desmos.com/calculator/rd2ohuwwzl
    vec4 color = v_Color * texture(texture_source, v_UV);
    vec3 P = v_Pos;
    vec3 E = eye;
    vec3 e_n = normalize(E - P);
    vec3 n = v_Normal;

    vec3 L = (view * vec4(light, 1.0)).xyz;

    vec3 l_n = normalize(P - L); // -norm(L - P);

    vec3 l_r = reflect(l_n, n);

    float b_spec = clamp(dot(e_n, l_r), 0.0, 1.0);
    float b_diff = clamp(dot(n, l_r) + 0.2, 0.2, 1.0);
    outColor = clamp((b_diff + b_spec) * light_colour * color, 0.0, 1.0);
    outColor.a = color.a;
}
