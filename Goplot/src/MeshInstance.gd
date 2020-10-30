extends MeshInstance

var rings = 50
var radial_segments = 50
var height = 1
var radius = 1

func _ready():
    var arr = []
    arr.resize(Mesh.ARRAY_MAX)

    # PoolVectorXXArrays for mesh construction.
    var verts = PoolVector3Array()
    var uvs = PoolVector2Array()
    var normals = PoolVector3Array()
    var indices = PoolIntArray()
    
    # Vertex indices.
    var thisrow = 0
    var prevrow = 0
    var point = 0

    # Loop over rings.
    for i in range(rings + 1):
        var v = float(i) / rings
        var w = sin(PI * v)
        var y = cos(PI * v)

        # Loop over segments in ring.
        for j in range(radial_segments):
            var u = float(j) / radial_segments
            var x = sin(u * PI * 2.0)
            var z = cos(u * PI * 2.0)
            var vert = Vector3(x * radius * w, y, z * radius * w)
            verts.append(vert)
            normals.append(vert.normalized())
            uvs.append(Vector2(u, v))
            point += 1

            # Create triangles in ring using indices.
            if i > 0 and j > 0:
                indices.append(prevrow + j - 1)
                indices.append(prevrow + j)
                indices.append(thisrow + j - 1)

                indices.append(prevrow + j)
                indices.append(thisrow + j)
                indices.append(thisrow + j - 1)

        if i > 0:
            indices.append(prevrow + radial_segments - 1)
            indices.append(prevrow)
            indices.append(thisrow + radial_segments - 1)

            indices.append(prevrow)
            indices.append(prevrow + radial_segments)
            indices.append(thisrow + radial_segments - 1)

        prevrow = thisrow
        thisrow = point

    # Assign arrays to mesh array.
    arr[Mesh.ARRAY_VERTEX] = verts
    arr[Mesh.ARRAY_TEX_UV] = uvs
    arr[Mesh.ARRAY_NORMAL] = normals
    arr[Mesh.ARRAY_INDEX] = indices

    # Create mesh surface from mesh array.
    #var mesh = Mesh.new()
    mesh = ArrayMesh.new()
    mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arr) # No blendshapes or compression used.
