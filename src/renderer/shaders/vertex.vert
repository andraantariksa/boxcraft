#version 450 core

layout(location = 0) in vec3 vertexPos;
layout(location = 1) in vec3 vertexNormal;
// Instance
layout(location = 10) in vec4 model1;
layout(location = 11) in vec4 model2;
layout(location = 12) in vec4 model3;
layout(location = 13) in vec4 model4;

layout(std140, set = 0, binding = 0) uniform Camera {
    mat4 projection;
    mat4 view;
    vec3 cameraPos;
};

layout(location = 0) out vec3 vertexPosOut;
layout(location = 1) out vec3 vertexNormalOut;

void main()
{
    mat4 model = mat4(
        model1,
        model2,
        model3,
        model4
    );
    vertexNormalOut = vertexNormal;
    vertexPosOut = vertexPos;
    gl_Position = projection * view * model * vec4(vertexPosOut, 1.0);
}
