extends Control
class_name ControlBenchmarkCustom

var num_lines := 5000
var rng := RandomNumberGenerator.new()
var max_size := 500.0

var label: Label


func _ready():
	print("ControlBenchmarkCustom")
	label = Label.new()
	label.anchor_right = 1.0
	label.horizontal_alignment = HORIZONTAL_ALIGNMENT_RIGHT
	label.grow_horizontal = Control.GROW_DIRECTION_BEGIN
	label.add_theme_color_override("font_color", Color(0.1, 0.1, 0.3))
	add_child(label)

	var shader = load("res://shaders/line.gdshader")

	var line_material = ShaderMaterial.new()
	line_material.shader = shader
	line_material.set_shader_parameter("width", 1.0)
	
	var xs = []
	var ys = []
	
	if false:
		for i in 10:
			xs.append(i * 20)
			ys.append(i * i * 5)
			
	if true:
		for i in 5000:
			xs.append(rng.randf_range(0, 500))
			ys.append(rng.randf_range(0, 500))
		
	var mesh_inst = ControlBenchmarkCustom.create_mesh_instance_lines(xs, ys, line_material)
	add_child(mesh_inst)


func _process(_delta):
	label.text = "FPS: %s" % Engine.get_frames_per_second()

	
static func build_triangles(points):
	var vertices = PackedVector2Array()
	var normals = PackedVector2Array()

	for i in points.size() - 1:
		var j = i + 1

		var p1 = points[i]
		var p2 = points[j]
		var d = p2 - p1
		var n = Vector2(d.y, -d.x).normalized()
		# HACK: encode the up/down bit in the length of the normal :(
		var n_up = +n
		var n_dn = -n * 2

		# print(p1, p2, d, n)

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


static func create_mesh_instance_lines(xs, ys, line_material):

	var points = PackedVector2Array()
	for i in range(len(xs)):
		points.push_back(Vector2(xs[i], ys[i]))

	var triangles = build_triangles(points)

	var arrays = []
	arrays.resize(ArrayMesh.ARRAY_MAX)
	arrays[ArrayMesh.ARRAY_VERTEX] = triangles.vertices
	arrays[ArrayMesh.ARRAY_TEX_UV] = triangles.normals

	# Create the Mesh
	var mesh = ArrayMesh.new()
	mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)
	# This doesn't seem to have an effect. It is weird anyway that
	# the material can be set on a Mesh (which is a resource) and
	# at the same time on the MeshInstance(2D) (which is an actual
	# CanvasItem that has a material).
	# mesh.surface_set_material(0, marker_material)

	var mesh_instance = MeshInstance2D.new()
	mesh_instance.mesh = mesh
	mesh_instance.material = line_material

	return mesh_instance
