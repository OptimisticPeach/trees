#version 320 es
precision mediump float;
uniform vec3 light;
uniform vec3 eye;
uniform highp mat4x4 view;
uniform highp mat4x4 model;
uniform vec4 light_colour;

in VertexData {
    vec4 f_Color;
    vec3 f_Pos;
    vec3 f_Normal;
    float f_Opacity;
} v_Out;
out vec4 outColor;

void main() {
    // https://www.desmos.com/calculator/rd2ohuwwzl
    vec4 color = v_Out.f_Color;
    vec3 P = v_Out.f_Pos;
    vec3 E = eye;
    vec3 e_n = normalize(E - P);
    vec3 n = v_Out.f_Normal;

    vec3 L = (view * vec4(light, 1.0)).xyz;

    vec3 l_n = normalize(P - L); // -norm(L - P);

    vec3 l_r = reflect(l_n, n);

    float b_spec = clamp(dot(e_n, l_r), 0.0, 1.0);
    float b_diff = clamp(dot(n, l_r) + 0.2, 0.2, 1.0);
    outColor.a = 1.0;
    outColor.rgb = (b_spec * light_colour).rgb;
    outColor.rgb += b_diff * color.a * color.rgb;
    outColor.a = max(b_spec, b_diff * color.a);
}
