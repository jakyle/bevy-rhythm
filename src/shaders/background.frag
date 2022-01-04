#version 450

// Creation, by Silexars (Danilo Guanabara)
// From https://www.shadertoy.com/view/XsXXDn

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float time;
};
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 resolution;
};

void main() {
    vec3 c;
    vec2 r = resolution;
    float l,z=time;
    for(int i=0; i<3; i++) {
        vec2 uv,p = v_Uv; // / r;
        uv = p;
        p -= 0.5;
        p.x *= r.x / r.y;
        z += 0.07;
        l = length(p);
        uv += p/l*(sin(z)+1.)*abs(sin(l*9.0-z*2.0));
        c[i] = (0.01)/length(abs(mod(uv,1.0)-0.5));
    }
    o_Target = vec4(c/l,time);
}