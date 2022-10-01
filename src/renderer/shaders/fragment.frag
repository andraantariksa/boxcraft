#version 450 core

layout(location = 0) in vec3 vertexPos;
layout(location = 1) in vec3 vertexNormal;
layout(location = 2) in vec2 vertexTextureCoord;
layout(location = 3) in flat ivec2 texturePos;

layout(std140, set = 0, binding = 0) uniform Camera {
    mat4 projection;
    mat4 view;
    vec3 cameraPos;
};

layout(set = 1, binding = 0) uniform texture2D textureAtlas;
layout(set = 1, binding = 1) uniform sampler textureAtlasSampler;
layout(std140, set = 1, binding = 2) uniform TextureAtlas {
    ivec2 textureAtlasSize;
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

    vec2 textureAtlasSlotSize = 1. / vec2(textureAtlasSize);
    vec2 textureCoord = mix(textureAtlasSlotSize * texturePos, textureAtlasSlotSize * vec2(texturePos + 1), vertexTextureCoord);
    vec3 col = texture(sampler2D(textureAtlas, textureAtlasSampler), textureCoord).rgb;
    //vec3 resultColor = (ambientColor + diffuseColor + specularColor) * col;
    //fragColor = vec4(resultColor, 1.);
    fragColor = vec4(col, 1.);
}
