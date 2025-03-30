use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type CanvasId = String;
pub type NodeId = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeInstance {
    pub id: String,
    pub node_id: NodeId,
    pub position: (f32, f32),
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Connection {
    pub from: (String, String), // nodeInstanceId, output
    pub to: (String, String),   // nodeInstanceId, input
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Canvas {
    pub id: CanvasId,
    pub name: String,
    pub node_instances: Vec<NodeInstance>,
    pub connections: Vec<Connection>,
}

