
#version 450

#extension GL_ARB_separate_shader_objects : enable

// vertex input
layout (location = 0) in vec2 inPosition;
layout (location = 1) in vec2 inTexCoord;

// vertex output
layout (location = 0) out vec2 fragTexCoord;

out gl_PerVertex {

    vec4 gl_Position;
};

void main() {

    gl_Position = vec4(inPosition, 0.0, 1.0);
    gl_Position.y = -gl_Position.y; // fix upside-down.

    fragTexCoord = inTexCoord;
}
