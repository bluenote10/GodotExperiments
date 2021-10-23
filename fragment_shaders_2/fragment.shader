shader_type canvas_item;
//render_mode skip_vertex_transform;

uniform mat4 global_transform;

// https://godotengine.org/qa/27367/how-to-get-fragment-world-coordinates-in-canvas-item-shader
// https://github.com/godotengine/godot/issues/19800
// https://www.reddit.com/r/godot/comments/8cgyi3/how_to_get_fragment_world_coordinates_in_canvas/

varying vec2 world_pos;

void vertex() {
    // VERTEX.xy seems to have values like (0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)
    // So the really interesting part is the WORLD_MATRIX
    // world_pos = VERTEX.xy;
	// world_pos = (WORLD_MATRIX * vec4(VERTEX, 0.0, 1.0)).xy;
    // But the problem is that the WORLD_MATRIX is not only MODEL => WORLD but also contains
    // WORLD => VIEW, so basically we get the same behavior as with FRAGCOORD/SCREEN_UV...
    
    world_pos = (global_transform * vec4(VERTEX, 0.0, 1.0)).xy;
    
    // VERTEX = (EXTRA_MATRIX * (WORLD_MATRIX * vec4(VERTEX, 0.0, 1.0))).xy;
}

void fragment() {
    
    float d = distance(world_pos, vec2(512, 300));
    float v = 1.0 - (d / 500.0);
    
    COLOR = vec4(v, v, v, 1.0);
}