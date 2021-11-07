shader_type canvas_item;
render_mode skip_vertex_transform;

uniform float width = 1.0;
uniform float antialias = 1.0;

uniform vec4 color: hint_color = vec4(1, 0, 0, 1);

void vertex() {
    float uv_length = length(UV);
    if (uv_length > 1.5) {
        UV *= 0.5;    
    }
    bool is_up = (uv_length < 1.5);

    VERTEX = (EXTRA_MATRIX * (WORLD_MATRIX * vec4(VERTEX, 0.0, 1.0))).xy;
    
    // Not sure if this is valid, but at least it seems to give better
    // results when lines fall exactly on pixels.
    VERTEX -= 0.5;

    float extension = (width / 2.0) + antialias;
    vec2 displacement = UV * extension;
    VERTEX.xy += displacement; // + 10.0 * vec2(cos(TIME), sin(TIME));
    UV = is_up ? vec2(+extension, 0.0) : vec2(-extension, 0.0);
}

void fragment() {
    float dist = abs(UV.x);
    float half_width = width / 2.0;
    
    float t = half_width - antialias;
    dist -= t;
    if( dist < 0.0 ) {
        COLOR = color;
    } else {
        dist /= antialias;
        COLOR = vec4(color.rgb, exp(-dist*dist) * color.a);
    }
}
