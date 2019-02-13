
#version 450

layout (location = 0) in vec3 inPos;
layout (location = 1) in vec3 inColor;

layout (set = 0, binding = 0) uniform UboView {
	mat4 projection;
	mat4 view;
	mat4 y_correction;
} uboView;

layout (set = 0, binding = 1) uniform UboInstance {
    mat4 model;
} uboInstance;

layout (location = 0) out vec3 outColor;

out gl_PerVertex {
    vec4 gl_Position;   
};


void main() {

	outColor = inColor;

	mat4 modelView = uboView.view * uboInstance.model;
	gl_Position = uboView.y_correction * uboView.projection * modelView * vec4(inPos.xyz, 1.0);
}
