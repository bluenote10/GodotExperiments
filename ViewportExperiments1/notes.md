
## Conclusion

The bug https://github.com/godotengine/godot/issues/23729 probably means that using the
standard ViewportContainer isn't ideal. Most likely implementing a custom viewport
controller is a cleaner solution than hacky work-arounds.
