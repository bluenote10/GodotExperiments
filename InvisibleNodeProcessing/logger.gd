"""
https://github.com/godotengine/godot/issues/24699

It looks like not being visible really doesn't stop nodes from
being 'active' in general. Disabling them seems to require manual
effort.
"""
extends Control


func _ready():
    print("%s _ready" % name)

    if name == "Parent":
        visible = false
        set_process(false)
        set_process_input(false)
        set_process_unhandled_input(false)
        set_process_unhandled_key_input(false)

func _process(delta):
    print("%s _process" % name)

func _input(event):
    print("%s _input" % name)

func _unhandled_input(event):
    print("%s _unhandled_input" % name)

func _gui_input(event):
    print("%s _gui_input" % name)

