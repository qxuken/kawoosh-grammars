#version 450 core

// A sample.
#define LIMIT 10

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 uv;
layout(location = 0) out vec4 colour;

uniform sampler2D tex;
uniform mat4 transform;
uniform float time;

struct Light {
    vec3 direction;
    float strength;
};

const float PI = 3.14159;

float area(float radius) {
    return PI * radius * radius;
}

vec3 shade(vec3 normal, Light light) {
    float d = max(dot(normalize(normal), -light.direction), 0.0);
    return vec3(d * light.strength);
}

void main() {
    Light light = Light(vec3(0.0, -1.0, 0.0), 0.8);
    vec4 base = texture(tex, uv);
    float total = 0.0;
    for (int i = 0; i < LIMIT; i++) {
        total += area(float(i)) * sin(time);
    }
    if (base.a < 0.5) {
        discard;
    }
    colour = vec4(base.rgb * shade(position, light), 1.0);
    gl_Position = transform * vec4(position, 1.0);
}
