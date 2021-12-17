extends Spatial


var multimesh: MultiMesh

const num_plates = 6
const num_cups = 8
const r1 = 0.3
const r2 = 0.9

var t = 0.0


func _ready():
    var mesh = CubeMesh.new()
    mesh.material = preload("res://material.tres")

    multimesh = MultiMesh.new()
    multimesh.transform_format = MultiMesh.TRANSFORM_3D
    multimesh.mesh = mesh
    multimesh.instance_count = num_cups * num_plates

    var multimesh_inst = MultiMeshInstance.new()
    multimesh_inst.multimesh = multimesh

    add_child(multimesh_inst)


func _process(delta):
    t += delta
    for i in num_cups:
        for j in num_plates:
            var phi1 = t
            var phi2 = t + float(i) / num_cups * 2 * PI
            var phi3 = t + float(j) / num_plates * 2 * PI
            # Since transform chaining is broken, we have to fallback to inefficient
            # and syntactically verbose transfrom multiplications
            var S = Transform.IDENTITY.scaled(Vector3(0.04, 0.04, 0.04))
            var R1 = Transform.IDENTITY.rotated(Vector3(0, 1, 0), phi1)
            var T1 = Transform.IDENTITY.translated(Vector3(r1, 0, 0))
            var R2 = Transform.IDENTITY.rotated(Vector3(0, 1, 0), phi2)
            var T2 = Transform.IDENTITY.translated(Vector3(r2, 0, 0))
            var R3 = Transform.IDENTITY.rotated(Vector3(0, 1, 0), phi3)
            # We need to think right-to-left here
            var T = R3 * T2 * R2 * T1 * R1 * S
            multimesh.set_instance_transform(i * num_plates + j, T)
