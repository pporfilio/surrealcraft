#version 330
in highp vec3 vert;
in highp vec3 vertexColor;
out highp vec4 fragColor;
void main() {
    fragColor = vec4(vertexColor, 1.0);
}
