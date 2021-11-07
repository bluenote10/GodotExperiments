shader_type canvas_item;

void vertex() {
    POINT_SIZE = 100.0;
}

void fragment() {
    // https://docs.godotengine.org/en/3.1/tutorials/shading/shading_reference/canvas_item_shader.html#fragment-built-ins
    //COLOR = vec4(texture(TEXTURE, UV).rgb, 1.0);
    
    // UV: normal texture
    COLOR = vec4(UV.x, UV.y, 0.0, 1.0);
    
    // SCREEN_UV: (0, 0) lower left (1, 1) upper right (with editor bug...)
    //COLOR = vec4(SCREEN_UV.x, SCREEN_UV.y, 0.0, 1.0);
    
    // For bug see: https://github.com/godotengine/godot/issues/33905
    
    // FRAGCOORD seems to be like SCREEN_UV, but pixels (with same editor bug...)
    //COLOR = vec4(FRAGCOORD.x / 1000.0, FRAGCOORD.y / 1000.0, 0.0, 1.0);
    //COLOR = vec4(FRAGCOORD.x / 1024.0, 0.0, 0.0, 1.0);
    //COLOR = vec4(0.0, FRAGCOORD.y / 600.0, 0.0, 1.0);
    
    // POINT_COORD: No idea, no effect
    //COLOR = vec4(POINT_COORD.y * 100.0, 0.0, 0.0, 1.0);
    
}