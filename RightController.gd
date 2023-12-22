extends MeshInstance3D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var rc = get_node("..") as XRController3D;
	var m = get_node(".") as MeshInstance3D 
	var sm = m.get_active_material(0) as StandardMaterial3D
	if rc.is_button_pressed("trigger"):
		sm.albedo_color = "green"
	else: 
		sm.albedo_color = "red"
	pass
