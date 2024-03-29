extends Control

var time_since_last_update := 0.0

var child_viewport: Viewport = null

func _ready():
    
    var direct = false
    
    if direct:
        var child = preload("res://Child.tscn").instance()
        add_child(child)
    
    else:
        var viewport_container = CustomViewportContainer.new()
        viewport_container.anchor_right = 1.0
        viewport_container.anchor_bottom = 1.0
        add_child(viewport_container)
        
        var viewport = Viewport.new()
        viewport.size = Vector2(500, 500)
        viewport.render_target_v_flip = true
        viewport.hdr = false
        viewport.usage = Viewport.USAGE_2D
        viewport.msaa = Viewport.MSAA_2X
        viewport.gui_disable_input = true
        viewport.render_target_update_mode = Viewport.UPDATE_ONCE
        viewport_container.add_child(viewport)
        
        var child = preload("res://Child.tscn").instance()
        viewport.add_child(child)
        
        child_viewport = viewport

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


func _process(delta):
    time_since_last_update += delta
    
    if time_since_last_update > 0.05:
        child_viewport.render_target_update_mode = Viewport.UPDATE_ONCE
        time_since_last_update = 0.0
