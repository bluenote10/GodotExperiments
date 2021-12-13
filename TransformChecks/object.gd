tool
extends Spatial


export(bool) var _reload = false setget _reload


func _reload(x):
    print("reloading tool")

    var T := Transform.IDENTITY.translated(Vector3(1, 0, 0))

    var S := Transform.IDENTITY.scaled(Vector3(2, 1, 0.5))

    var R := Transform.IDENTITY.rotated(Vector3(0, 1, 0), 1.5)

    var base = T*R*S

    transform = base

    # Any S * R produces an "invalid" transform:
    # transform = S * R
    # transform = R.scaled(Vector3(2, 1, 0.5))
    # transform = Transform.IDENTITY.rotated(Vector3(0, 1, 0), PI/4).scaled(Vector3(0.5, 1, 1))

