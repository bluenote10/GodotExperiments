extends Spatial

"""
Check based on MultiMesh.

Works as expected as well. Bascially the main difference is that
MultiMeshes don't have a material override (only a color override).
"""

var material_a = preload("res://material_a.tres")

var mesh = null

var dt := 0.0

func _ready():
    mesh = SphereMesh.new()
    mesh.height = 0.4
    mesh.radius = 0.2

    mesh.surface_set_material(0, material_a)

    var multimesh = MultiMesh.new()

    multimesh.mesh = mesh
    multimesh.transform_format = MultiMesh.TRANSFORM_3D
    multimesh.instance_count = 2
    
    multimesh.set_instance_transform(0, get_tf(Vector3(-0.2, 0, 0)))
    multimesh.set_instance_transform(1, get_tf(Vector3(+0.2, 0, 0)))

    var multimesh_instance = MultiMeshInstance.new()
    multimesh_instance.multimesh = multimesh
    
    add_child(multimesh_instance)


func get_tf(pos):
    return Transform.IDENTITY.translated(pos)


func _process(delta):
    dt += delta
    
    if dt > 1.0:
        dt -= 1.0
        
        material_a.albedo_color = Color(randf(), randf(), randf())
        
        mesh.height *= 1.1
        
