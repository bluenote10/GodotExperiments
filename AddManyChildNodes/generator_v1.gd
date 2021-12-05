extends Spatial

"""
With spheres:

N =  5000       0.039 s       0.008 ms/op
N = 10000       0.079 s       0.008 ms/op
N = 15000       0.121 s       0.008 ms/op

With cubes:

N =  5000       0.036 s       0.007 ms/op
N = 10000       0.075 s       0.007 ms/op
N = 15000       0.114 s       0.008 ms/op

Generating like this seems very linear, and definitely fast.

Failed to reproduce slow performance like that...

Numbers below were obtained with an initial version that still performed
the Mesh.new() in the add_many_children function, which is MUCH slower
and also shows a big difference between spheres and cubes.

With spheres:

N =  250       0.657 s
N =  500       1.250 s
N =  750       1.905 s
N = 1000       2.547 s       2.547 ms/op
N = 1250       3.171 s
N = 1500       3.793 s
N = 1750       4.402 s
N = 2000       5.103 s       2.551 ms/op
N = 3000       7.613 s       2.538 ms/op
N = 4000      10.141 s       2.535 ms/op

With cubes:

N = 4000       0.194 s       0.049 ms/op
N = 8000       0.397 s       0.050 ms/op

"""

const num_nodes := 5000

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
    mesh_inst.mesh = mesh
    mesh_inst.translate(position)

    parent.add_child(mesh_inst)


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
