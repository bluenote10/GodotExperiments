extends Spatial


func place_cube(phi):
    var mesh = CubeMesh.new()
    mesh.material = preload("res://material.tres")

    var mesh_inst = MeshInstance.new()
    mesh_inst.mesh = mesh

    var S = Transform.IDENTITY.scaled(Vector3(0.08, 0.08, 0.08))
    var T = Transform.IDENTITY.translated(Vector3(0.8, 0, 0))
    var R = Transform.IDENTITY.rotated(Vector3(0, 1, 0), phi)

    mesh_inst.transform = R * T * S

    add_child(mesh_inst)


func place_bar(phi):
    var mesh = CubeMesh.new()
    mesh.material = preload("res://material.tres")

    var mesh_inst = MeshInstance.new()
    mesh_inst.mesh = mesh

    var S = Transform.IDENTITY.scaled(Vector3(1, 0.08, 0.08))
    var T = Transform.IDENTITY.translated(Vector3(0, 0, 0))
    var R = Transform.IDENTITY.rotated(Vector3(0, 1, 0), phi)

    mesh_inst.transform = T * R * S

    add_child(mesh_inst)


func _ready():

    var n = 20

    for i in n:
        place_cube(float(i) / n * 2 * PI)
        #place_bar(float(i) / n * 2 * PI)
