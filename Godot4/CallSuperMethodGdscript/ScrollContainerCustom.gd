extends ScrollContainer
class_name ScrollContainerSwallowMousewheel

func _input(event):
	# Promote certain high-priority events from the `_gui_input` stage to `_input` to hide
	# them from all GUI children.
	if event is InputEventMouseButton and event.is_pressed():
		if event.button_index == MOUSE_BUTTON_WHEEL_UP or event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			super._gui_input(event)
			accept_event()
