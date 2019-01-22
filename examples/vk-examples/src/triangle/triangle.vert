
#version 450

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inColor;

layout (binding = 0) uniform UBO {
	mat4 projectionMatrix;
	mat4 modelMatrix;
	mat4 viewMatrix;
	mat4 y_correction;
} ubo;

layout (location = 0) out vec3 outColor;

out gl_PerVertex {
    vec4 gl_Position;   
};


void main() {

	outColor = inColor;
	gl_Position = ubo.y_correction * ubo.projectionMatrix * ubo.viewMatrix * ubo.modelMatrix * vec4(inPos.xyz, 1.0);
}
