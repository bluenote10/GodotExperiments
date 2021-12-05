extends Spatial

"""
Ah, found it, it is the legible_unique_name parameter of the add_child function, which
apparently leads to quadratic complexity internally...

With spheres:

N =  1000       0.444 s       0.444 ms/op
N =  2000       5.046 s       2.523 ms/op
N =  3000      29.617 s       9.872 ms/op

It should be noted that the node names without enabling legible_unique_name are still
pretty readable as long as one sets their name.
"""

const num_nodes := 1000

var added := false
var t: = 0.0

var cube_mesh = CubeMesh.new()
var sphere_mesh = SphereMesh.new()


func _ready():
    cube_mesh.size = Vector3(0.05, 0.05, 0.05)

    sphere_mesh.height = 0.1
    sphere_mesh.radius = 0.05


func add_many_children(parent: Node, position: Vector3, kind = "sphere"):
    var mesh: Mesh
    if kind == "cube":
        mesh = cube_mesh
    elif kind == "sphere":
        mesh = sphere_mesh

    var mesh_inst = MeshInstance.new()
    mesh_inst.name = "SomeName"
    mesh_inst.mesh = mesh
    mesh_inst.translate(position)

    parent.add_child(mesh_inst, true)


func _process(delta):
    t += delta

    if t > 3.0 and not added:
        added = true

        print("Start adding")
        var t1 = OS.get_ticks_msec()

        for i in num_nodes:
            var position = Vector3(randf(), randf(), randf())
            add_many_children(self, position)

        var t2 = OS.get_ticks_msec()
        print("Stop adding")

        var t_sec = float(t2 - t1) / 1000
        print("N = %5d    %8.3f s    %8.3f ms/op" % [num_nodes, t_sec, t_sec * 1000 / num_nodes])
