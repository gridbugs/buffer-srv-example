#version 150 core

uniform samplerBuffer t_TexBuf;

out vec4 Target0;

void main() {
    vec4 colour = texelFetch(t_TexBuf, 0);
    Target0 = colour;
}
