#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;

varying lowp vec2 f_uv;
varying lowp vec3 f_color;
varying lowp vec3 f_normal;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    f_uv = texcoord;
    f_color = color0.rgb / 255.0;
    f_normal = mat3(Model) * normal.xyz;
}