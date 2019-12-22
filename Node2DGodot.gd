extends Node2D


func _ready():
    
    var size = 512
    
    var image = Image.new()
    image.create(size, size, false, Image.FORMAT_RGB8)
    
    var texture = ImageTexture.new()
    texture.create(size, size, image.get_format(), 0)
    texture.set_data(image)
    
    var sprite = Sprite.new()
    sprite.set_texture(texture)
    sprite.set_name("Sprite")
    
    add_child(sprite, true)
    
    
func _process(delta):
    
    var texture = $Sprite.get_texture()
    var image = texture.get_data()
    
    image.lock()
    for i in image.get_width():
        for j in image.get_height():
            var value = 0.5 # rand_range(0.0, 1.0)
            image.set_pixel(i, j, Color(value, value, value, 1))
    image.unlock()
            
    texture.set_data(image)

