extends Label

func _process(delta):
	var fps = Engine.get_frames_per_second()
	text = "FPS: " + str(int(fps))
