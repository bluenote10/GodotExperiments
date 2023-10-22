extends Node

var last_time: int = -1

func _ready():
	var device_ids = Input.get_connected_joypads()
	print("device_ids: ", device_ids)
	for i in device_ids:
		print(Input.get_joy_name(i), ", ", Input.get_joy_guid(i))
		
	# Looks like Godot actually has an entry in its mapping database for my GUID:
	# 03000000790000001100000010010000,Retro Controller,a:b1,b:b2,back:b8,dpdown:+a1,dpleft:-a0,dpright:+a0,dpup:-a1,leftshoulder:b6,lefttrigger:b7,rightshoulder:b4,righttrigger:b5,start:b9,x:b0,y:b3,platform:Linux,
	#
	# Relevant source in Godot is `joypad_linux.cpp`.

func _input(event: InputEvent):
	var curr_time := Time.get_ticks_usec()
	
	if event is InputEventJoypadButton and event.is_pressed():
		if last_time != -1:
			var delta_ms := float(curr_time - last_time) / 1000
			var fps := Engine.get_frames_per_second()
			print("delta = %10.3f ms    fps = %8.1f fps    %s" % [delta_ms, fps, event])
		
	last_time = curr_time
	

