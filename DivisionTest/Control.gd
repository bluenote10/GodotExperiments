extends Control

func div(a, b):
    return a / b

func f(x: float):
    print(typeof(x) == TYPE_REAL)   # should be called TYPE_FLOAT

func normalize_values(values, reference_value):
    var new_values = []
    for value in values:
        new_values.append(value * 1.0 / reference_value)
    return new_values

#func midpoint(a: float, b: float):
func midpoint(a, b):
    return 1.0 * (a + b) / 2


func _ready():
    #print(midpoint(1.0, 4.0))
    #print(midpoint(1, 4))
    #print(midpoint(Vector2(1.0, 2.0), Vector2(3.0, 5.0)))

    var x = 0

    f(42)

    print(normalize_values([5, 3, 4], 3))
    print(normalize_values([2.1, 6.2, 4.2], 2.1))
    print(normalize_values([Vector2(1, 1), Vector2(2, 3)], Vector2(2, 2)))
    print(normalize_values([Vector3(1, 1, 1), Vector3(2, 3, 4)], Vector3(2, 2, 2)))
