#version 450 core

layout(location = 0) in vec3 vertexPos;
layout(std140, set = 0, binding = 0) uniform Camera {
    mat4 projection;
    mat4 view;
};

//out vec4 vertexColor;

void main()
{
    // projection * view *
    gl_Position = view * vec4(vertexPos, 1.0);
    //    vertexColor = vec4(0.5, 0.0, 0.0, 1.0);
}
