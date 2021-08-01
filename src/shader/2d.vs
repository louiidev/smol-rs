#version 330 core
layout (location = 0) in vec2 vertex;
layout (location = 1) in vec2 tex_coords;

out vec2 TexCoord;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 view;

void main()
{
    gl_Position = projection * view * model * vec4(vertex.x, 1-vertex.y, 0.0, 1.0);
    TexCoord = tex_coords;
}