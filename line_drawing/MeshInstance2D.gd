tool
extends MeshInstance2D


func get_normal(points, i, close):
    var min_index = 0
    var max_index = points.size() - 1
    var prev = i - 1
    var next = i + 1
    if prev < min_index:
        if close:
            prev = max_index
        else:
            var d = points[next] - points[i]
            return Vector2(-d.y, d.x).normalized()
    if next > max_index:
        if close:
            next = min_index
        else:
            var d = points[i] - points[prev]
            return Vector2(-d.y, d.x).normalized()
    
    var d_prev = points[i] - points[prev]
    var d_next = points[next] - points[i]
    var n_prev = Vector2(-d_prev.y, d_prev.x).normalized()
    var n_next = Vector2(-d_next.y, d_next.x).normalized()

    var alpha = acos(n_prev.dot(n_next))
    var required_len = 1.0 / cos(0.5 * alpha)
    var n = (n_prev + n_next).normalized() * required_len
    # print(i, d_prev, d_next, n_prev, n_next, alpha, required_len)
    return n


func build_triangles(points, close=true):
    var vertices = PoolVector2Array()
    var normals = PoolVector2Array()
    var colors = PoolColorArray()
    
    var max_i = points.size() if close else points.size() - 1
    
    for i in max_i:
        var j = i + 1
        if j >= points.size():
            j = 0

        var p1 = points[i]
        var p2 = points[j]
        #var d = p2 - p1
        #var n = Vector2(d.y, -d.x).normalized()
        var n1 = get_normal(points, i, close)
        var n2 = get_normal(points, j, close)
        
        var c_outside = Color(0, 0, 0, 0)
        var c_inside = Color(0, 0, 0, 1)
        # print(p1, p2, d, n)
        # print(p1, p2, n1, n2)

        # triangle 1
        vertices.push_back(p1)
        vertices.push_back(p2)
        vertices.push_back(p1)
        normals.push_back(n1)
        normals.push_back(n2)
        normals.push_back(n1)
        colors.push_back(c_outside)
        colors.push_back(c_outside)
        colors.push_back(c_inside)

        # triangle 2
        vertices.push_back(p1)
        vertices.push_back(p2)
        vertices.push_back(p2)
        normals.push_back(n1)
        normals.push_back(n2)
        normals.push_back(n2)
        colors.push_back(c_inside)
        colors.push_back(c_outside)
        colors.push_back(c_inside)
        
    
    return {
        vertices=vertices, 
        normals=normals,
        colors=colors,
    }


func _ready():

    var points = PoolVector2Array()
    points.push_back(Vector2(0, 0))
    points.push_back(Vector2(100, 0))
    points.push_back(Vector2(100, 100))
    points.push_back(Vector2(0, 200))

    var triangles = build_triangles(points)
    
    var arrays = []
    arrays.resize(ArrayMesh.ARRAY_MAX)
    arrays[ArrayMesh.ARRAY_VERTEX] = triangles.vertices
    arrays[ArrayMesh.ARRAY_TEX_UV] = triangles.normals
    arrays[ArrayMesh.ARRAY_COLOR] = triangles.colors
    
    # Create the Mesh.
    var mesh = ArrayMesh.new()
    mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)

    set_mesh(mesh)
