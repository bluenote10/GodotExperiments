extends Control

"""

Conclusion:

When nodes are on the same level (i.e., siblings) mouse events seem to go
to the node further down in the tree, IF it is visible AND the event
overlaps with its area.

This is actually kind of weird, because normally events aren't area specific.

Also, the other ControlB here gets the events forwarded as input just fine,
which means that the UI elements should also see them.

What has the visibility of ControlA to do with the event handling of the
covered siblings? Somehow ControlA must mark the event in some way, or even
the fact that it has been processed is relevant. But no, even if the
input processing in ControlA is disabled, its mere visibility stops the event
from being propagated to the UI components.

"""

onready var subcontrol_a = $ControlA
onready var subcontrol_b = $ControlB

func _ready():
    check_processing_state(subcontrol_a)


func _input(event):
    print("root: ", event)

    if event.is_action_pressed("ui_accept"):
        subcontrol_a.visible = !subcontrol_a.visible
        check_processing_state(subcontrol_a)

    if event.is_action_pressed("enable_processing"):
        subcontrol_a.set_process_input(true)
        check_processing_state(subcontrol_a)

    if event.is_action_pressed("disable_processing"):
        subcontrol_a.set_process_input(false)
        check_processing_state(subcontrol_a)


func check_processing_state(node: Node):
    print("visible: ", node.visible)
    print("is_processing: ", node.is_processing())
    print("is_processing_input: ", node.is_processing_input())
    print("is_processing_internal: ", node.is_processing_internal())
    print("is_processing_unhandled_input: ", node.is_processing_unhandled_input())
    print("is_processing_unhandled_key_input: ", node.is_processing_unhandled_key_input())
