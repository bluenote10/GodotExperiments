"""
Goal: Model relevant functionality of

https://github.com/godotengine/godot/blob/master/scene/gui/subviewport_container.cpp
"""

extends Container
class_name CustomViewportContainer


func _ready():
    pass


func _draw():
    print("redrawing viewport container")
    for i in get_child_count():
        var child = get_child(i) as Viewport
        if child != null:
            var size = Vector2(100, 100) # rect_size
            draw_texture_rect(child.get_texture(), Rect2(Vector2(0, 0), rect_size), false)
            print(child.get_texture())
            print(child.get_texture().get_size())        
