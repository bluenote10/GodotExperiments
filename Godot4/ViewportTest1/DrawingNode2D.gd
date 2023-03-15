extends Node2D

var t := 0.0

func _draw():
	draw_line(Vector2(0.5, 0.5), Vector2(200.5, 0.5), Color.GREEN_YELLOW, 0.5, true)
	draw_line(Vector2(0.5, 0.5), Vector2(0.5, 100.5), Color.GREEN_YELLOW, 0.5, true)
	draw_line(Vector2(0.5, 0.5), Vector2(200.5, 100.5), Color.GREEN_YELLOW, 0.5, true)
	var x = 50.5 + 50 * cos(t)
	var y = 50.5 + 50 * sin(t)
	draw_line(Vector2(50.5, 50.5), Vector2(x, y), Color.DARK_RED, 2.0, true)
	

func _process(delta):
	t += delta
	queue_redraw()
