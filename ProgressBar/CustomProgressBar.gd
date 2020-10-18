extends Control

export var texture_h_border := 8
export var texture_w_border := 6

export var progress := 0.5 setget set_progress, get_progress

signal progress_change_request(progress, is_final)

onready var nine_patch_bg := $Background as NinePatchRect
onready var nine_patch_fg := $Foreground as NinePatchRect


func _ready():
    setup_nine_patch_rect(nine_patch_bg)
    setup_nine_patch_rect(nine_patch_fg)
    
    set_custom_minimum_size(Vector2(2*texture_w_border, 2*texture_h_border))

    resize_fg_to_current_value()
        

func _notification(what):
    match what:
        NOTIFICATION_RESIZED:
            resize_fg_to_current_value()

func _gui_input(event):
    # print("_gui_input: ", event)
    # print(event.is_pressed())
    # print(Input.is_mouse_button_pressed(1))
    
    var is_button = event is InputEventMouseButton
    var is_drag = event is InputEventMouseMotion and Input.is_mouse_button_pressed(1)
    
    if is_button or is_drag:
        var x = event.position.x

        var total_width = rect_size.x
        var available_width = total_width - 2.0 * texture_w_border
        
        progress = (x - texture_w_border) / available_width
        
        if progress < 0.0:
            progress = 0.0
        elif progress > 1.0:
            progress = 1.0
            
        resize_fg_to_current_value()

        var is_final = is_button and not event.is_pressed()
        emit_signal("progress_change_request", progress, is_final)
    
    # Pros/cons of calling this?
    accept_event()
    

func get_progress():
    return progress
    

func set_progress(value):
    progress = value
    resize_fg_to_current_value()
    

func setup_nine_patch_rect(nine_patch: NinePatchRect):
    nine_patch.patch_margin_left = texture_w_border
    nine_patch.patch_margin_right = texture_w_border
    nine_patch.patch_margin_top = texture_h_border
    nine_patch.patch_margin_bottom = texture_h_border
    
    nine_patch.margin_top = -texture_h_border
    nine_patch.margin_bottom = +texture_h_border
     
    
func resize_fg_to_current_value():
    var total_width = rect_size.x
    var available_width = total_width - 2.0 * texture_w_border
    nine_patch_fg.margin_right = - (1.0 - progress) * available_width
           

"""
# For debug drawing the Control boundaries
func _draw():
    draw_rect(Rect2(Vector2(0, 0), get_size()), Color(1, 0, 0), false)
"""
