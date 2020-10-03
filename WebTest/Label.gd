extends Label

func _process(delta):
    var pos := get_global_mouse_position()
    text = "Hello World %s %s" % [pos.x, pos.y]
