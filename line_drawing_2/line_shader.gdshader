shader_type canvas_item;

uniform float width = 1.0;
uniform float antialias = 1.0;

uniform vec4 color: hint_color = vec4(1, 0, 0, 1);

void vertex() {
    float uv_length = length(UV);
    if (uv_length > 1.5) {
        UV *= 0.5;    
    }
    bool is_up = (uv_length < 1.5);

    VERTEX.x -= 0.5;

    float extension = (width / 2.0) + antialias;
    vec2 displacement = UV * extension;
    VERTEX.xy += displacement; // + 10.0 * vec2(cos(TIME), sin(TIME));
    UV = is_up ? vec2(+extension, 0.0) : vec2(-extension, 0.0);
}

void fragment() {
    float dist = abs(UV.x);
    float half_width = width / 2.0;
    if (dist < half_width) {
        COLOR = color;
    } else {
        float scale = (dist - half_width) / antialias;
        COLOR = vec4(color.rgb, 1.0 - scale);
    }
}
