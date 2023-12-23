/*
 lets create the equivalent of

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


*/

use godot::prelude::*;

struct RustOpenXR {}

#[gdextension]
unsafe impl ExtensionLibrary for RustOpenXR {}

use godot::engine::Node;
use godot::engine::INode;
use godot::engine::Material;
use godot::engine::MeshInstance3D;
use godot::engine::StandardMaterial3D;
use godot::engine::XrController3D;

#[derive(GodotClass)]
#[class(base=Node)]
struct TriggerColorChanger {
    node: Base<Node>,
}

#[godot_api]
impl INode for TriggerColorChanger {
    fn init(owner: Base<Node>) -> Self {
        TriggerColorChanger { node: owner }
    }

    fn process(&mut self, _delta: f64) {
        // our parent is the XRController3D we use to figure out button presses
        let orc: Option<Gd<XrController3D>> = self.node.try_get_node_as("..");
        if let Some(rc) = orc {
            // our sibling is the MeshInstance3D we use to change the material color
            let om: Option<Gd<MeshInstance3D>> = self.node.try_get_node_as("../ControllerMesh");
            if let Some(m) = om {
                let osm: Option<Gd<Material>> = m.get_active_material(0);
                if let Some(sm) = osm {
                    let ostm = sm.try_cast::<StandardMaterial3D>();
                    if let Ok(mut stm) = ostm {
                        if rc.is_button_pressed("trigger".into()) {
                            stm.set_albedo(Color::from_rgb(0.0, 1.0, 1.0));
                        } else {
                            stm.set_albedo(Color::from_rgb(1.0, 1.0, 1.0));
                        }
                    } else {
                        godot_print!("no standard material on mesh");
                    }
                } else {
                    godot_print!("no material on mesh");
                }
            } else {
                godot_print!("no controller mesh");
            }
        } else {
            godot_print!("no XRController3D parent");
        }
    }
}
