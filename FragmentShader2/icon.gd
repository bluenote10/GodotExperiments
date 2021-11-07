tool
extends Sprite

func _ready():
    print("Duplicating material")
    material = material.duplicate()

func _process(_delta):
    material.set_shader_param("global_transform", get_global_transform())
    