# Souces:
# https://godotassetstore.org/assets/3d/shaders/gh:BastiaanOlij:godot-sky-asset
# https://github.com/BastiaanOlij/godot-sky-asset

extends Spatial


func _on_Sky_texture_sky_updated():
    var env = get_viewport().get_camera().environment
    print(env)
    $Sky_texture.copy_to_environment(env)
