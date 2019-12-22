extends Label

func _process(delta):
    text = "%.1f" % Engine.get_frames_per_second()
    print(Engine.get_frames_per_second())