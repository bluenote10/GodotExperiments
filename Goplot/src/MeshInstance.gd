extends MeshInstance


const default_material = preload("res://resources/default_material.tres")


func create_mesh(data):
    var arr = []
    arr.resize(Mesh.ARRAY_MAX)

    # PoolVectorXXArrays for mesh construction.
    var verts = PoolVector3Array()
    var normals = PoolVector3Array()
    var indices = PoolIntArray()

    var num_rows = len(data)
    var num_cols = len(data[0])
    
    var index = 0
    
    for i in num_rows:
        for j in num_cols:
            verts.append(Vector3(i, data[i][j], j))
            normals.append(Vector3(rand_range(0, 1), rand_range(0, 1), rand_range(0, 1)).normalized())
            
            if i > 0 and j > 0:
                indices.append(index - num_cols - 1)
                indices.append(index - 1)
                indices.append(index)

                indices.append(index - num_cols - 1)
                indices.append(index)
                indices.append(index - num_cols)

            index += 1
    
    
    # Assign arrays to mesh array.
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
