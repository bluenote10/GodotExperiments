extends Node2D

var child: Sprite2D
var font: Font

func _ready():
	child = Sprite2D.new()
	child.texture = load("res://icon.svg")
	child.transform.origin = Vector2(100, 100)
	#add_child(child)
	
	font = SystemFont.new()
	
	
func _draw():
	# Alignment semantics are kind of broken...
	# https://github.com/godotengine/godot/issues/80163
	var s = "Hello World"
	var w = font.get_string_size(s).x
	draw_rect(Rect2(100, 100, 5, 5), Color.RED, true)
	draw_rect(Rect2(100, 140, 5, 5), Color.RED, true)
	draw_rect(Rect2(100, 180, 5, 5), Color.RED, true)
	draw_string(font, Vector2(100, 100), s, HORIZONTAL_ALIGNMENT_CENTER)
	draw_string(font, Vector2(100-w/2, 140), s, HORIZONTAL_ALIGNMENT_LEFT)
	draw_string(font, Vector2(100-w, 180), s, HORIZONTAL_ALIGNMENT_RIGHT)
	
	draw_set_transform(Vector2(200, 200))
	draw_rect(Rect2(100, 100, 5, 5), Color.RED, true)
	draw_rect(Rect2(100, 140, 5, 5), Color.RED, true)
	draw_rect(Rect2(100, 180, 5, 5), Color.RED, true)
	draw_string(font, Vector2(100, 100), s, HORIZONTAL_ALIGNMENT_CENTER)
	draw_string(font, Vector2(100-w/2, 140), s, HORIZONTAL_ALIGNMENT_LEFT)
	draw_string(font, Vector2(100-w, 180), s, HORIZONTAL_ALIGNMENT_RIGHT)

	draw_set_transform(Vector2(400, 400), 0.1)
	draw_rect(Rect2(100, 100, 5, 5), Color.RED, true)
	draw_rect(Rect2(100, 140, 5, 5), Color.RED, true)
	draw_rect(Rect2(100, 180, 5, 5), Color.RED, true)
	draw_string(font, Vector2(100, 100), s, HORIZONTAL_ALIGNMENT_CENTER)
	draw_string(font, Vector2(100-w/2, 140), s, HORIZONTAL_ALIGNMENT_LEFT)
	draw_string(font, Vector2(100-w, 180), s, HORIZONTAL_ALIGNMENT_RIGHT)


