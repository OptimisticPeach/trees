#version 320 es
layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;
//layout(line_strip, max_vertices = 4) out;

in PerVertex {
    vec4 g_Color;
    vec3 g_Pos;
    float f_Opacity;
} v_In[];

out VertexData {
    vec4 f_Color;
    vec3 f_Pos;
    vec3 f_Normal;
    float f_Opacity;
} v_Out;

void main() {
    v_Out.f_Normal = normalize(cross(v_In[1].g_Pos - v_In[0].g_Pos, v_In[2].g_Pos - v_In[0].g_Pos));

    v_Out.f_Color = v_In[0].g_Color;
    v_Out.f_Pos = v_In[0].g_Pos;
    v_Out.f_Opacity = v_In[0].f_Opacity;
    gl_Position = gl_in[0].gl_Position;
    EmitVertex();

    v_Out.f_Color = v_In[1].g_Color;
    v_Out.f_Pos = v_In[1].g_Pos;
    v_Out.f_Opacity = v_In[1].f_Opacity;
    gl_Position = gl_in[1].gl_Position;
    EmitVertex();

    v_Out.f_Color = v_In[2].g_Color;
    v_Out.f_Pos = v_In[2].g_Pos;
    v_Out.f_Opacity = v_In[2].f_Opacity;
    gl_Position = gl_in[2].gl_Position;
    EmitVertex();

//    v_Out.f_Color = v_In[0].g_Color;
//    v_Out.f_Pos = v_In[0].g_Pos
//    v_Out.f_Opacity = v_In[0].f_Opacity;
//    gl_Position = gl_in[0].gl_Position;
//    EmitVertex();
}
