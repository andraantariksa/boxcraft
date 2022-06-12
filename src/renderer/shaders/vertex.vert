#version 450 core

layout(location = 0) in vec3 vertexPos;
layout(location = 1) in vec3 vertexNormal;
layout(std140, set = 0, binding = 0) uniform Camera {
    mat4 projection;
    mat4 view;
    vec3 cameraPos;
};

layout(location = 0) out vec3 vertexPosOut;
layout(location = 1) out vec3 vertexNormalOut;

void main()
{
    // projection * view *
    vertexNormalOut = vertexNormal;
    vertexPosOut = vertexPos;
    gl_Position = projection * view * vec4(vertexPosOut, 1.0);
}
