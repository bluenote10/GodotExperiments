extends CanvasLayer

export var game_world : NodePath
export (float, 0.1, 2.0) var resolution_scale = 1.0 setget change_resolution_scale
export var texture_filtering = true setget change_filtering
export var ssaa_enabled = true setget change_enabled
onready var ssaa = get_node("SuperSampling")
onready var shader = ssaa.material
var viewport
var viewport_size
var root_viewport
var native_res
var original_res

func _ready():
	var game_node = get_node(game_world)
	get_parent().call_deferred("remove_child", game_node)
	get_screen_size()
	viewport = Viewport.new()
	viewport.name = "Viewport"
	viewport.size = native_res
	viewport.usage = Viewport.USAGE_2D
	viewport.render_target_clear_mode = Viewport.CLEAR_MODE_ALWAYS
	viewport.render_target_update_mode = Viewport.UPDATE_ALWAYS
	viewport.render_target_v_flip = true
	viewport.size_override_stretch = true
	viewport.call_deferred("add_child", game_node)
	set_shader_texture()
	get_parent().call_deferred("add_child", viewport)
	original_res = native_res
	root_viewport = get_viewport()
	viewport.connect("size_changed", self, "on_window_resize")
	root_viewport.connect("size_changed", self, "on_window_resize")
	on_window_resize()

func set_shader_texture():
	yield(VisualServer, "frame_post_draw")
	var view_texture = viewport.get_texture()
	view_texture.flags = Texture.FLAG_FILTER if texture_filtering else 0
	view_texture.viewport_path = viewport.get_path()
	shader.set_shader_param("viewport", view_texture)
	shader.set_shader_param("enabled", ssaa_enabled)
	change_resolution_scale(resolution_scale)
	
func set_shader_resolution():
	if shader:
		shader.set_shader_param("resolution", viewport_size)
	
func on_window_resize():
	get_screen_size()
	set_viewport_size()
	resize_viewport()
	scale_viewport_canvas()
	set_shader_resolution()
	
func get_screen_size():
	var window = OS.window_size
	native_res = window

func set_viewport_size():
	viewport_size = native_res * resolution_scale
	
func resize_viewport():
	if viewport:
		viewport.size = viewport_size
	
func scale_viewport_canvas():
	if viewport:
		viewport.set_size_override(true, original_res)
		
func change_resolution_scale(res):
	resolution_scale = res
	on_window_resize()

func change_filtering(filter):
	texture_filtering = filter
	if viewport:
		var view_texture = viewport.get_texture()
		view_texture.flags = Texture.FLAG_FILTER if texture_filtering else 0
		
func change_enabled(enable):
	ssaa_enabled = enable
	if shader:
		shader.set_shader_param("enabled", ssaa_enabled)
