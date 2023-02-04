extends Control


onready var itemlist = $ItemList as ItemList


func _ready():
    for i in 1000:
        itemlist.add_item("File %d" %i)


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#    pass
