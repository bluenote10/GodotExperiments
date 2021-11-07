"""
Goal: Model relevant functionality of

https://github.com/godotengine/godot/blob/master/scene/gui/subviewport_container.cpp
"""

extends Container
class_name CustomViewportContainer


func _ready():
    pass


func _process(_delta):
    # update()
    pass
    

func _draw():
    print("redrawing viewport container")
    
    for i in get_child_count():
        var child = get_child(i) as Viewport
        if child != null:
            var texture = child.get_texture()
            var size = texture.get_size() # rect_size
            draw_texture_rect(child.get_texture(), Rect2(Vector2(0, 0), size), false)
