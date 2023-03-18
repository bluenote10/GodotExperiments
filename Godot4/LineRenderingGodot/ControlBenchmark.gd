extends Control

var num_lines := 5000
var rng := RandomNumberGenerator.new()
var max_size := 500.0

var label: Label


func _ready():
	label = Label.new()
	label.anchor_right = 1.0
	label.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
	label.grow_horizontal = Control.GROW_DIRECTION_BEGIN
	label.add_theme_color_override("font_color", Color(0.1, 0.1, 0.3))
	add_child(label)


func _process(_delta):
	label.text = "FPS: %s" % Engine.get_frames_per_second()
	# Note that it doesn't make a (significant) difference if the drawing content
	# is dynamic or static.
	# queue_redraw()


func _draw():
	for _i in num_lines:
		var x_1 = rng.randf_range(0, max_size)
		var y_1 = rng.randf_range(0, max_size)
		var x_2 = rng.randf_range(0, max_size)
		var y_2 = rng.randf_range(0, max_size)
		draw_line(Vector2(x_1, y_1), Vector2(x_2, y_2), Color(1, 0, 0), 0.5, true)
