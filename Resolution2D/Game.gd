extends Control

func _input(event):
    if event.is_action_pressed("fullscreen"):
        OS.window_fullscreen = not OS.window_fullscreen
