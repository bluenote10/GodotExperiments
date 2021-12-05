extends Label


func _ready():
    # The trick to right-aligning a label is to set its anchor to the
    # right, change its alignment mode to right aligned, and (IMPORTANT)
    # also change the horizontal grow direction from 'end' (growing
    # to the right) to 'begin' (growing to the left).
    anchor_right = 1.0
    align = Label.ALIGN_RIGHT
    grow_horizontal = Control.GROW_DIRECTION_BEGIN
    
    add_color_override("font_color", Color(0.1, 0.1, 0.3))


func _process(_delta: float) -> void:
    text = "%.1f FPS" % Engine.get_frames_per_second()
