extends Node2D

func _ready():
    $Node2DRust.queue_free()
    #$Node2DGodot.queue_free()

    # To check actual precision of floats:
    print("%.16f" % PI)
    print("%.16f" % (1.0 / 3.0))
    
    # Results after going through `to_json`
    print(to_json(PI))
    print(to_json(1.0 / 3.0))

    if true:
        var values = []
        var num_to_generate = 1000000
        
        #  0.8333333767950535 to 1.0000000521540642
        var from = 0.6666667014360428
        var upto = 0.8333333767950535 
        for i in num_to_generate:
            values.append(rand_range(from, upto))

        var values_str = PoolStringArray()
        for x in values:
            values_str.append("%.16f" % x)
        
        #var output = "["
        #for x in values:
        #    output += "%.16f" % x
        #output += "]"
        var output = "[" + values_str.join(", ") + "]"
        
        var save_game = File.new()
        save_game.open("user://debug_data.save", File.WRITE)
        #save_game.store_line(to_json(values))
        save_game.store_line(output)
        save_game.close()
