
#version 450

#extension GL_ARB_separate_shader_objects: enable

layout (set = 0, binding = 0) uniform UboOjbect {
    mat4 projection;
    mat4 view;
    mat4 model;
    mat4 y_correction; // `y_correction` is used to fix y-coordinate upside-down.
} ubo;

layout (set = 0, binding = 1) uniform DynNode {
	mat4 transform;
} dyn_node;

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {

    vec4 model = ubo.model * dyn_node.transform * vec4(inPosition, 1.0);
    gl_Position = ubo.y_correction * ubo.projection * ubo.view * model;
}
