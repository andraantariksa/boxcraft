#version 450 core

layout(location = 0) in vec3 vertexPos;
layout(location = 1) in vec3 vertexNormal;
layout(std140, set = 0, binding = 0) uniform Camera {
    mat4 projection;
    mat4 view;
    vec3 cameraPos;
};

layout(location = 0) out vec4 fragColor;

void main()
{
    const vec3 lightColor = vec3(1.);
    const vec3 light_position = vec3(0., 5., 5.);

    // Ambient
    const float ambientStrength = .1;
    vec3 ambientColor = lightColor * ambientStrength;

    // Diffuse
    vec3 lightDir = normalize(light_position - vertexPos);
    float diffuseStrength = max(dot(vertexNormal, lightDir), 0.0);
    vec3 diffuseColor = lightColor * diffuseStrength;

    // Specular
    vec3 viewDir = normalize(cameraPos - vertexPos);
    vec3 reflectDir = reflect(-lightDir, vertexNormal);
    const float specularStrength = .5;
    float specular = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specularColor = specularStrength * specular * lightColor;

    vec3 resultColor = (ambientColor + diffuseColor + specularColor) * vec3(1.0, 0.0, 0.0);
    fragColor = vec4(resultColor, 1.);
}
