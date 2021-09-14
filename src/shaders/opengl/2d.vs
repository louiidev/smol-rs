#version 330 core

layout (location = 0) in vec4 vertex;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 tex_coords;
layout (location = 3) in float tex_index;

out vec2 TexCoord;
out vec4 v_color;
out float v_texture_index;

uniform mat4 projection_view;

void main()
{
    gl_Position = projection_view * vertex;
    TexCoord = tex_coords;
    v_color = color;
    v_texture_index = tex_index;
}

