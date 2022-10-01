#version 450 core

layout(location = 0) in vec3 vertexPos;
layout(location = 1) in vec3 vertexNormal;
layout(location = 2) in vec2 vertexTextureCoord;
// Instance
layout(location = 10) in vec4 model1;
layout(location = 11) in vec4 model2;
layout(location = 12) in vec4 model3;
layout(location = 13) in vec4 model4;
layout(location = 14) in ivec2 texturePos;

layout(std140, set = 0, binding = 0) uniform Camera {
    mat4 projection;
    mat4 view;
    vec3 cameraPos;
};

layout(location = 0) out vec3 vertexPosOut;
layout(location = 1) out vec3 vertexNormalOut;
layout(location = 2) out vec2 vertexTextureCoordOut;
layout(location = 3) out flat ivec2 texturePosOut;

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
    vertexTextureCoordOut = vertexTextureCoord;
    texturePosOut = texturePos;
    gl_Position = projection * view * model * vec4(vertexPosOut, 1.0);
}
