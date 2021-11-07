extends Control


func _ready():
    # The trick to right-aligning a label is to set its anchor to the
    # right, change its alignment mode to right aligned, and (IMPORTANT)
    # also change the horizontal grow direction from 'end' (growing
    # to the right) to 'begin' (growing to the left).
    var fps_label = Label.new()
    fps_label.set_script(preload("res://fps.gd"))
    fps_label.anchor_right = 1.0
    fps_label.align = Label.ALIGN_RIGHT
    fps_label.grow_horizontal = Control.GROW_DIRECTION_BEGIN
    add_child(fps_label)
    
    var direct = false
    
    if direct:
        var child = preload("res://Child.tscn").instance()
        add_child(child)
    
    else:
        var viewport_container = ViewportContainer.new()
        viewport_container.anchor_right = 1.0
        viewport_container.anchor_bottom = 1.0
        add_child(viewport_container)
        
        var viewport = Viewport.new()
        viewport.size = Vector2(400, 400)
        viewport.hdr = false
        viewport.usage = Viewport.USAGE_2D
        viewport.msaa = Viewport.MSAA_2X
        viewport.gui_disable_input = true
        viewport.render_target_update_mode = Viewport.UPDATE_DISABLED
        viewport_container.add_child(viewport)
        
        var child = preload("res://Child.tscn").instance()
        viewport.add_child(child)

        # Hack around bug: https://github.com/godotengine/godot/issues/23729
        yield(get_tree(), "idle_frame")
        viewport.render_target_update_mode = Viewport.UPDATE_ONCE


