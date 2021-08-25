#version 150

out vec4 FragColor;  
in vec2 TexCoord;

uniform sampler2D ourTexture;
uniform vec4 u_color;


void main()
{
    FragColor = texture(ourTexture, TexCoord) * u_color; 
}