extends Spatial

"""
This check sets a material both on Mesh and on MeshInstance.

Nothing unexpected here. Setting it on the MeshInstance simply works as an
override.
"""

var material_a = preload("res://material_a.tres")
var material_b = preload("res://material_b.tres")

var mesh = null

var dt := 0.0

func _ready():
    mesh = SphereMesh.new()
    mesh.height = 0.4
    mesh.radius = 0.2

    mesh.surface_set_material(0, material_a)

    add_sphere(-0.2, false)
    add_sphere(+0.2, true)


func add_sphere(x, with_override):
    var mesh_inst = MeshInstance.new()
    mesh_inst.mesh = mesh
    mesh_inst.translate(Vector3(x, 0, 0))

    if with_override:
        mesh_inst.set_surface_material(0, material_b)
    
    assert(mesh.get_surface_count() == mesh_inst.get_surface_material_count())

    add_child(mesh_inst)
    

func _process(delta):
    dt += delta
    
    if dt > 1.0:
        dt -= 1.0
        
        material_b.albedo_color = Color(randf(), randf(), randf())
        
        mesh.height *= 1.1
        
