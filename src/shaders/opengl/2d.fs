#version 330 core

out vec4 FragColor; 
 
in vec2 TexCoord;
in vec4 v_color;
in float v_texture_index;

uniform sampler2D u_textures[32];


void main()
{
    int index = int(v_texture_index);
    FragColor = texture(u_textures[index], TexCoord) * v_color;
}