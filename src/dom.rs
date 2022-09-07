use std::collections::{HashMap, HashSet};

// Main node data struct
struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

// This can be appended with more node types
enum NodeType {
    Text(String),
    Element(ElementData),
}

// Would be better organised using namespaces
struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(), node_type: NodeType::Text(data)
    }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
