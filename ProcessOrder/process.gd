extends Node


var num_frames := 0


func _ready():
    print("_ready [%s]" % name)


func _input(_event):
    var delta = get_process_delta_time()
    print("_input frame = %d, delta = %s [%s] event: %s" % [num_frames, delta, name, _event.as_text()])


func _process(delta):
    # dummy load:
    var sum := 0.0
    for _i in 1000:
        for _j in 1000:
            sum += 0.1

    num_frames += 1
    print("_process frame = %d, delta = %s [%s]" % [num_frames, delta, name])
