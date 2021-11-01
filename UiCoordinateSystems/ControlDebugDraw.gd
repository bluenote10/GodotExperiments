extends Control


func _draw():
    # Regarding the drawing coordinates, it doesn't seem necessary/appropriate
    # to incorporate rect_position, which only seems to be used internally
    # for positioning the Control w.r.t. its parent.
    draw_rect(Rect2(Vector2(0, 0), rect_size), Color(1, 1, 1), false, 3.0)
    draw_rect(Rect2(Vector2(0, 0), Vector2(10, 10)), Color(1, 0, 0))
    draw_rect(Rect2(Vector2(90, 90), Vector2(10, 10)), Color(0, 1, 0))
