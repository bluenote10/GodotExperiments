extends Control


var rng = RandomNumberGenerator.new()


func _ready():
    rng.randomize()
    
    
func _process(delta):
    update()
    # pass
    
    
func _draw():
    
    var num_lines = 1000
    
    for i in num_lines:
        var x1 = rng.randf_range(0.0, 500.0);
        var y1 = rng.randf_range(0.0, 500.0);
        var x2 = rng.randf_range(0.0, 500.0);
        var y2 = rng.randf_range(0.0, 500.0);
        draw_line(Vector2(x1, y1), Vector2(x2, y2), Color(0, 0, 1))

    draw_line(Vector2(0, 0), Vector2(100, 100), Color(0, 1, 0))
