extends Node2D

var t := 0.0

func _process(delta):
    t += delta
    
    var sx = 1.0 + sin(t) * 0.8
    var sy = 1.0 + sin(t) * 0.8
    
    transform = Transform2D(Vector2(sx, 0), Vector2(0, sy), Vector2(0, 0))
    
