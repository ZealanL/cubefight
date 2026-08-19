#version 100
precision mediump float;

varying lowp vec2 f_uv;
varying lowp vec3 f_color;
varying lowp vec3 f_normal;

void main() {
    vec3 base_color = vec3(0.7, 0.8, 0.9);
    vec3 highlight_color = vec3(0.3, 0.2, 0.1);
    vec3 sun_normal = normalize(vec3(0.3, 0.9, 0.1));
    float sun_scale = dot(f_normal, sun_normal);
    vec3 color = base_color + (highlight_color * sun_scale);

    gl_FragColor = vec4(color * f_color, 1.0);
}