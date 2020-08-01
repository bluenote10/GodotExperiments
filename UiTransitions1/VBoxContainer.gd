extends VBoxContainer


onready var panel := $Control/PanelContainer as PanelContainer
onready var tween := $Tween as Tween

var state_visible := true

const ANIM_TIME = 0.3

func _input(event):
    if event.is_action_pressed("toggle_menu"):
        print("toggle")
        if state_visible:
            hide_panel()
            state_visible = false
        else:
            show_panel()
            state_visible = true


func _on_Button1_pressed():
    print("<<")
    #panel.margin_left -= 100
    #panel.margin_right -= 100
    panel.rect_position.x -= 100


func _on_Button2_pressed():
    print("center")
    panel.rect_position.x = 0


func _on_Button3_pressed():
    print(">>")
    #panel.margin_left += 100
    #panel.margin_right += 100
    panel.rect_position.x += 100


func hide_panel():
    # var w = panel.rect_size.x
    var pos_from = panel.rect_position.x
    var pos_upto = 30
    tween.interpolate_property(
        panel, "rect_position:x", 
        pos_from, pos_upto, 
        ANIM_TIME, Tween.TRANS_CUBIC, Tween.EASE_IN)
    tween.interpolate_property(
        panel, "modulate:a", 
        panel.modulate.a, 0.0, 
        ANIM_TIME, Tween.TRANS_CUBIC, Tween.EASE_IN)        
    tween.start()


func show_panel():
    var pos_from = panel.rect_position.x
    var pos_upto = 0
    tween.interpolate_property(
        panel, "rect_position:x", 
        pos_from, pos_upto, 
        ANIM_TIME, Tween.TRANS_CUBIC, Tween.EASE_OUT)
    tween.interpolate_property(
        panel, "modulate:a", 
        panel.modulate.a, 1.0, 
        ANIM_TIME, Tween.TRANS_CUBIC, Tween.EASE_OUT)        
    tween.start()
    
