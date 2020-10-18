extends Control


func _on_Control_progress_change_request(progress, is_final):
    print("progress_change_request: ", progress, " ", is_final)
