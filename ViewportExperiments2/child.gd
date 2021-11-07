extends Control


var rng = RandomNumberGenerator.new()


func _ready():
    rng.randomize()
    
    
func _process(delta):
    # update()
    pass
    
    
func _draw():
    # Without calling update:
    # 10.0 million =>   5 fps
    #  1.0 million =>  36 fps
    #  0.1 million => 144 fps
    # With calling update:
    #  100 k =>  10 fps
    #   10 k =>  91 fps
    #    1 k => 144 fps
    
    var num_lines = 1000000
    
    """
    for i in num_lines:
        var x1 = rng.randf_range(0.0, 500.0);
        var y1 = rng.randf_range(0.0, 500.0);
        var x2 = rng.randf_range(0.0, 500.0);
        var y2 = rng.randf_range(0.0, 500.0);
        draw_line(Vector2(x1, y1), Vector2(x2, y2), Color(0, 1, 0))
    """
    draw_line(Vector2(0, 0), Vector2(100, 100), Color(0, 1, 0))
