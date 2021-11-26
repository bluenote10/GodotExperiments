extends Spatial

"""
In contrast to Impl1, this uses a single Mesh.

1000 objects (when not resetting the instance count)
FPS: 480
Init: 0.002 sec

1000 objects (when setting the instance count in every frame)
FPS: 380
Init: 0.002 sec

10000 objects (when not resetting the instance count)
FPS: 60
Init: 0.134 sec

10000 objects (when setting the instance count in every frame)
FPS: 42
Init: 0.023 sec

Observation from 'Monitors':
- Objects drawn is N
- Draw calls is 2, i.e., significantly lower!
- Surface changes is 2
- Mat changes is 2
- Shader changes is 2

Note: Updating the instance transforms does not seem to impact performance,
in contrast to setting the instance count.

Conclusions:
- MultiMesh does reduce draw calls, but isn't per se faster.
- MultiMesh is potentially slower if the number of elements changes a lot.
- MultiMesh can handle per-instance updates without a significant performance
  overhead, which is somewhat surprising.
"""

class State:
    var id: int
    var position: Vector3
    var color: Color
    
    func _init(_id: int, _position: Vector3, _color: Color):
        id = _id
        position = _position
        color = _color
        
const num_meshes = 10000

var states = []
var id_counter = 0

var state_to_node = {}

var mesh = SphereMesh.new()
var multi_mesh: MultiMesh
var multi_mesh_instance: MultiMeshInstance


func _ready():
    mesh.height = 0.1
    mesh.radius = 0.05
    
    multi_mesh = MultiMesh.new()
    multi_mesh.mesh = mesh
    multi_mesh.transform_format = MultiMesh.TRANSFORM_3D
    
    multi_mesh_instance = MultiMeshInstance.new()
    multi_mesh_instance.multimesh = multi_mesh
    
    add_child(multi_mesh_instance)

    for i in num_meshes:
        var new_state = State.new(
            id_counter, Vector3(randf(), randf(), randf()), Color(randf(), randf(), randf()))
        id_counter += 1
        states.append(new_state)        


func diff_state_to_node():
    var any_change = false
    var t1 = OS.get_ticks_msec()
    
    if multi_mesh.instance_count != len(states): # or true:
        any_change = true
    
        multi_mesh.instance_count = len(states)
        
        for i in len(states):
            var state = states[i]
            var transform = Transform.IDENTITY.translated(state.position)
            multi_mesh.set_instance_transform(i, transform)

    var fake_transform_invalidation = false
    if fake_transform_invalidation:
        var transform = Transform.IDENTITY.translated(Vector3(randf(), randf(), randf()))
        multi_mesh.set_instance_transform(0, transform)
        

    var t2 = OS.get_ticks_msec()
    if any_change:
        print("Time to diff:", t2 - t1)


func _process(_delta):
    diff_state_to_node()
    
    
