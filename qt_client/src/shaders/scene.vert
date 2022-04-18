#version 330
layout(location = 0) in vec4 vertex;
void main() {
    gl_Position = vec4(vertex.x, vertex.y, vertex.z, 1.0);
}
