"""
This implementation is simplified over the line_drawing_1:
- It doesn't compute the "angle bisector" normals, because miter
  joins can "explode".
- Removed the option to "close" a point segment. Can always be emulated
  by adding the first point as last.
- Single color.
"""
tool
extends MeshInstance2D


func build_triangles(points):
    var vertices = PoolVector2Array()
    # We must abuse colors for passing the "normals", because we
    # need an additional bit to indicate the normal direction.
    # FUCK: Colors are limited to positive values only => not possible either...
    var normals = PoolColorArray()
    # var normals = PoolVector3Array()
    
    for i in points.size() - 1:
        var j = i + 1

        var p1 = points[i]
        var p2 = points[j]
        var d = p2 - p1
        var n = Vector2(d.y, -d.x).normalized()
        var n_up = Color(+n.x, +n.y, +1)
        var n_dn = Color(-n.x, -n.y, -1)
        
        # print(p1, p2, d, n)
        # print(p1, p2, n1, n2)

        # triangle 1
        vertices.push_back(p1)
        vertices.push_back(p2)
        vertices.push_back(p1)
        normals.push_back(n_up)
        normals.push_back(n_up)
        normals.push_back(n_dn)

        # triangle 2
        vertices.push_back(p1)
        vertices.push_back(p2)
        vertices.push_back(p2)
        normals.push_back(n_dn)
        normals.push_back(n_up)
        normals.push_back(n_dn)
        
    
    return {
        vertices=vertices, 
        normals=normals,
    }


func _ready():

    var points = PoolVector2Array()
    points.push_back(Vector2(0, 5))
    points.push_back(Vector2(100, 0))
    points.push_back(Vector2(100, 100))
    points.push_back(Vector2(0, 200))

    var triangles = build_triangles(points)
    
    var arrays = []
    arrays.resize(ArrayMesh.ARRAY_MAX)
    arrays[ArrayMesh.ARRAY_VERTEX] = triangles.vertices
    arrays[ArrayMesh.ARRAY_COLOR] = triangles.normals
    
    # Create the Mesh
    var mesh = ArrayMesh.new()
    mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)

    set_mesh(mesh)
