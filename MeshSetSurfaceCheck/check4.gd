extends Spatial

"""
Check what happens if one changes the material that a mesh
points to.
"""

var material_a = preload("res://material_a.tres")
var material_b = preload("res://material_b.tres")

var mesh = null
var mesh_inst_l = null
var mesh_inst_r = null

var dt := 0.0

func _ready():

    mesh = SphereMesh.new()
    mesh.height = 0.4
    mesh.radius = 0.2

    mesh.surface_set_material(0, material_a)

    mesh_inst_l = add_sphere(-0.3, false)
    mesh_inst_r = add_sphere(+0.3, true)


func add_sphere(x, with_override):
    var mesh_inst = MeshInstance.new()
    mesh_inst.mesh = mesh
    mesh_inst.translate(Vector3(x, 0, 0))

    if with_override:
        mesh_inst.set_surface_material(0, material_a)
    
    assert(mesh.get_surface_count() == mesh_inst.get_surface_material_count())

    add_child(mesh_inst)
    return mesh_inst
    

func _process(delta):
    dt += delta
    
    if dt > 1.0:
        dt -= 1.0
        # This affects only the left one, which has no override
        # mesh.surface_set_material(0, material_b)
        
        # This also only affects only the left one, basically creating
        # an override for it on the fly, i.e., it doesn't matter that it
        # had no override and it doesn't affect the original mesh.
        mesh_inst_l.set_surface_material(0, material_b)
        
        # This only modifies the override of the right one:
        # mesh_inst_r.set_surface_material(0, material_b)
        
