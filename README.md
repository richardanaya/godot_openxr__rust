# Godot OpenXR using Rust

![image](https://github.com/richardanaya/godot_openxr_simple_button/assets/294042/1f462ebe-d10d-4c92-8554-11e6bd5fb052)

A simple project to show how to do VR with PBR.

Look at the Main.gd to see how the OpenXR session is started ( this is just boiler plate from the website tutorials).

Importantly in the project settings I set renderer to Forward+ and turned on XR and XR Shaders (important!)

# Rust code to have trigger change color

```rust
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

```
