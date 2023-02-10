use crate::utils::gd_add_child;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct InstantiationOrder {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl GodotExt for InstantiationOrder {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("InstantiationOrder::init called");
        // add_subtree_entire_tree(&mut base.share());
        // add_subtree_root_first(&mut base.share());
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("InstantiationOrder::ready called");
        // add_subtree_entire_tree(&mut self.base.share());
        add_subtree_root_first(&mut self.base.share());
        godot_print!("InstantiationOrder::ready finished");
    }
}

fn add_subtree_entire_tree(parent: &mut Gd<Node>) {
    let root = Gd::<LoggingNode>::with_base(|_| LoggingNode::new("root".to_string()));
    let child_a = Gd::<LoggingNode>::with_base(|_| LoggingNode::new("child_a".to_string()));
    let child_b = Gd::<LoggingNode>::with_base(|_| LoggingNode::new("child_b".to_string()));

    gd_add_child!(root.share().upcast::<Node>(), child_a);
    gd_add_child!(root.share().upcast::<Node>(), child_b);
    // Note: If we don't add the root, the children's ready will never be called.
    // Probably we would simply leak the three nodes if we wouldn't call .free()
    // explicitly.
    gd_add_child!(parent, root);

    /*
    Order when calling this function from InstantiationOrder::init():

        InstantiationOrder::init called
        LoggingNode::init [root] called
        LoggingNode::init [child_a] called
        LoggingNode::init [child_b] called
        LoggingNode::ready [child_a] called
        LoggingNode::ready [child_b] called
        LoggingNode::ready [root] called
        InstantiationOrder::ready called
        InstantiationOrder::ready finished

    Order when calling this function from InstantiationOrder::ready():

        InstantiationOrder::init called
        InstantiationOrder::ready called
        LoggingNode::init [root] called
        LoggingNode::init [child_a] called
        LoggingNode::init [child_b] called
        LoggingNode::ready [child_a] called
        LoggingNode::ready [child_b] called
        LoggingNode::ready [root] called
        InstantiationOrder::ready finished

    Conclusion:
    - When adding a subtree, ready is called from children to parents.
    - The order of the init calls does not affect anything. The call order of the children is
      determined by the order when they are added to the scene tree.
    - By postponing the scene tree modification in the parent class (moving it to ready),
      the parent class gets a chance to execute things both before and after the subtree
      enters. When adding the subtree to self.base in init already, the ready function of
      the parent runs after all other ready functions, again due to the child-to-parent
      call order.
    */
}

fn add_subtree_root_first(parent: &mut Gd<Node>) {
    let root = Gd::<LoggingNode>::with_base(|_| LoggingNode::new("root".to_string()));
    let child_a = Gd::<LoggingNode>::with_base(|_| LoggingNode::new("child_a".to_string()));
    let child_b = Gd::<LoggingNode>::with_base(|_| LoggingNode::new("child_b".to_string()));

    gd_add_child!(parent, root.share());
    gd_add_child!(root.share().upcast::<Node>(), child_a);
    gd_add_child!(root.share().upcast::<Node>(), child_b);

    /*
    Order when calling this function from InstantiationOrder::init():

        InstantiationOrder::init called
        LoggingNode::init [root] called
        LoggingNode::init [child_a] called
        LoggingNode::init [child_b] called
        LoggingNode::ready [child_a] called
        LoggingNode::ready [child_b] called
        LoggingNode::ready [root] called
        InstantiationOrder::ready called
        InstantiationOrder::ready finished

    Order when calling this function from InstantiationOrder::ready():

        InstantiationOrder::init called
        InstantiationOrder::ready called
        LoggingNode::init [root] called
        LoggingNode::init [child_a] called
        LoggingNode::init [child_b] called
        LoggingNode::ready [root] called
        LoggingNode::ready [child_a] called
        LoggingNode::ready [child_b] called
        InstantiationOrder::ready finished

    Conclusion:
    - When calling from init, the order gets again reversed to child-to-parent order,
      i.e., it doesn't really matter that we are adding the root first, because the
      parent itself isn't part of the scene tree yet. When the parent gets added, Godot
      again calls ready from bottom to top.
    - Only when adding the root first from the ready function the order actually changes,
      because now the parent is in the scene tree, and thus, the root can be added immediately,
      i.e., it's ready can be called right away. Postponing adding the children then leads
      to postponing their ready calls.
    */
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LoggingNode {
    name: String,
}

impl LoggingNode {
    fn new(name: String) -> Self {
        godot_print!("LoggingNode::init [{}] called", name);
        Self { name }
    }
}

#[godot_api]
impl GodotExt for LoggingNode {
    /*
    fn init(mut base: Base<Self::Base>, name: String) -> Self {
        godot_print!("LoggingNode::init [{}] called", name);
        Self { name }
    }
    */

    fn ready(&mut self) {
        godot_print!("LoggingNode::ready [{}] called", self.name);
    }
}
