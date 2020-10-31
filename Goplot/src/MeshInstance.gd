extends MeshInstance


const default_material = preload("res://resources/default_material.tres")


func get_normal(data, i, j, z_scale, grid_step_i=1, grid_step_j=1):
    var normal = Vector3.ZERO

    var len_i = len(data)
    var len_j = len(data[0])
    
    if i > 0:
        var delta_y = z_scale * (data[i-1][j] - data[i][j])
        normal += Vector3(-delta_y, grid_step_i, 0).normalized()
    if i < len_i - 1:
        var delta_y = z_scale * (data[i][j] - data[i+1][j])
        normal += Vector3(-delta_y, grid_step_i, 0).normalized()
        
    if j > 0:
        var delta_y = z_scale * (data[i][j-1] - data[i][j])
        normal += Vector3(0, grid_step_j, -delta_y).normalized()
    if j < len_j - 1:
        var delta_y = z_scale * (data[i][j] - data[i][j+1])
        normal += Vector3(0, grid_step_j, -delta_y).normalized()

    return normal.normalized()        


func coord(idx, axis_len):
    return float(idx) - float(axis_len) / 2


func create_mesh(data, z_scale=3.0):

    var verts = PoolVector3Array()
    var normals = PoolVector3Array()
    var indices = PoolIntArray()

    var len_i = len(data)
    var len_j = len(data[0])
    
    var index = 0
    
    for i in len_i:
        for j in len_j:
            var x = coord(i, len_i)
            var z = coord(j, len_j)
            var y = z_scale * data[i][j]
            verts.append(Vector3(x, y, z))
            normals.append(get_normal(data, i, j, z_scale))
            
            if i > 0 and j > 0:
                indices.append(index - len_j - 1)
                indices.append(index - 1)
                indices.append(index)

                indices.append(index - len_j - 1)
                indices.append(index)
                indices.append(index - len_j)

            index += 1
    
    var arr = []
    arr.resize(Mesh.ARRAY_MAX)
    arr[Mesh.ARRAY_VERTEX] = verts
    arr[Mesh.ARRAY_NORMAL] = normals
    arr[Mesh.ARRAY_INDEX] = indices

    # Create mesh surface from mesh array.
    mesh = ArrayMesh.new()
    mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arr)

    # These two are equivalent, but note the inconsistent naming:
    # https://github.com/godotengine/godot/issues/16863#issuecomment-652067884
    mesh.surface_set_material(0, default_material)
    # set_surface_material(0, default_material)
