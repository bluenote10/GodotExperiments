extends Control

@onready var ui := $Ui

func _ready():
	ui.hello_world("from GDScript")
