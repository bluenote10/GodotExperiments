extends VBoxContainer


onready var panel := $Control/PanelContainer as PanelContainer
onready var tween := $Tween as Tween

var state_visible := true


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
    var w = panel.rect_size.x
    var pos_from = panel.rect_position.x
    var pos_upto = w
    tween.interpolate_property(
        panel, "rect_position:x", 
        pos_from, pos_upto, 
        0.3, Tween.TRANS_CUBIC, Tween.EASE_IN)
    tween.start()


func show_panel():
    var w = panel.rect_size.x
    var pos_from = panel.rect_position.x
    var pos_upto = 0
    tween.interpolate_property(
        panel, "rect_position:x", 
        pos_from, pos_upto, 
        0.3, Tween.TRANS_CUBIC, Tween.EASE_OUT)
    tween.start()
    
