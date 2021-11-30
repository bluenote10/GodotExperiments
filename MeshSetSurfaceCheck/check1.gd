extends Spatial

"""

This check allows to toggle setting the material either on the mesh or the mesh
instance. On first glance, both approaches behave identical.

Other observations:
- Modifying the material leads to changes.
- Modifying the mesh leads to changes.

"""

var material_a = preload("res://material_a.tres")

var set_material_on = "mesh"    # "mesh" / "mesh_inst"

var mesh = null

var dt := 0.0

func _ready():
    print_surface_counts()

    mesh = SphereMesh.new()
    mesh.height = 0.4
    mesh.radius = 0.2

    if set_material_on == "mesh":
        mesh.surface_set_material(0, material_a)

    add_sphere(-0.2)
    add_sphere(+0.2)


func print_surface_counts():
    """
    In retrospect this is relatively boring, because all PrimitiveMeshes
    must have a surface count of 1.
    """
    var plane = PlaneMesh.new()
    var cube = CubeMesh.new()
    var sphere = SphereMesh.new()
    var cylinder = CylinderMesh.new()
    
    print("plane: %s" % plane.get_surface_count())
    print("cube: %s" % cube.get_surface_count())
    print("sphere: %s" % sphere.get_surface_count())
    print("cylinder: %s" % cylinder.get_surface_count())
        

func add_sphere(x):
    var mesh_inst = MeshInstance.new()
    mesh_inst.mesh = mesh
    mesh_inst.translate(Vector3(x, 0, 0))

    if set_material_on == "mesh_inst":
        mesh_inst.set_surface_material(0, material_a)
    
    assert(mesh.get_surface_count() == mesh_inst.get_surface_material_count())

    add_child(mesh_inst)
    

func _process(delta):
    dt += delta
    
    if dt > 1.0:
        dt -= 1.0
        
        material_a.albedo_color = Color(randf(), randf(), randf())
        
        mesh.height *= 1.1
        
