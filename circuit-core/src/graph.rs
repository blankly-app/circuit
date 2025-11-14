use crate::{error::CircuitError, error::Result, value::Value};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Unique identifier for a node in the graph
pub type NodeId = String;

/// A node in the execution graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node instance
    pub id: NodeId,
    /// Type of block this node represents
    pub block_type: String,
    /// Node-specific configuration
    pub config: HashMap<String, Value>,
    /// Display position (for visual editors)
    pub position: Option<(f64, f64)>,
}

/// Connection between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Source node ID
    pub from_node: NodeId,
    /// Source output port
    pub from_port: String,
    /// Target node ID
    pub to_node: NodeId,
    /// Target input port
    pub to_port: String,
}

/// A directed graph of nodes and connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    /// Graph identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Graph description
    pub description: Option<String>,
    /// Nodes in the graph
    pub nodes: HashMap<NodeId, Node>,
    /// Connections between nodes
    pub connections: Vec<Connection>,
}

impl Graph {
    /// Create a new empty graph
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            nodes: HashMap::new(),
            connections: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(CircuitError::Graph(format!(
                "Node with id '{}' already exists",
                node.id
            )));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        if !self.nodes.contains_key(node_id) {
            return Err(CircuitError::NodeNotFound(node_id.to_string()));
        }

        // Remove all connections involving this node
        self.connections
            .retain(|conn| conn.from_node != node_id && conn.to_node != node_id);

        self.nodes.remove(node_id);
        Ok(())
    }

    /// Add a connection between two nodes
    pub fn add_connection(&mut self, connection: Connection) -> Result<()> {
        // Validate nodes exist
        if !self.nodes.contains_key(&connection.from_node) {
            return Err(CircuitError::NodeNotFound(connection.from_node.clone()));
        }
        if !self.nodes.contains_key(&connection.to_node) {
            return Err(CircuitError::NodeNotFound(connection.to_node.clone()));
        }

        // Check for cycles
        if self.would_create_cycle(&connection)? {
            return Err(CircuitError::CycleDetected);
        }

        self.connections.push(connection);
        Ok(())
    }

    /// Check if adding a connection would create a cycle
    fn would_create_cycle(&self, new_connection: &Connection) -> Result<bool> {
        let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();

        // Build adjacency list with existing connections
        for conn in &self.connections {
            adjacency
                .entry(conn.from_node.as_str())
                .or_default()
                .push(conn.to_node.as_str());
        }

        // Add the new connection
        adjacency
            .entry(new_connection.from_node.as_str())
            .or_default()
            .push(new_connection.to_node.as_str());

        // Check for cycle using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        fn has_cycle(
            node: &str,
            adjacency: &HashMap<&str, Vec<&str>>,
            visited: &mut HashSet<String>,
            rec_stack: &mut HashSet<String>,
        ) -> bool {
            visited.insert(node.to_string());
            rec_stack.insert(node.to_string());

            if let Some(neighbors) = adjacency.get(node) {
                for &neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        if has_cycle(neighbor, adjacency, visited, rec_stack) {
                            return true;
                        }
                    } else if rec_stack.contains(neighbor) {
                        return true;
                    }
                }
            }

            rec_stack.remove(node);
            false
        }

        for node_id in self.nodes.keys() {
            if !visited.contains(node_id.as_str())
                && has_cycle(node_id, &adjacency, &mut visited, &mut rec_stack)
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Get nodes in topological order (execution order)
    pub fn topological_sort(&self) -> Result<Vec<NodeId>> {
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();

        // Initialize in-degree for all nodes
        for node_id in self.nodes.keys() {
            in_degree.insert(node_id, 0);
        }

        // Build adjacency list and calculate in-degrees
        for conn in &self.connections {
            adjacency
                .entry(conn.from_node.as_str())
                .or_default()
                .push(conn.to_node.as_str());
            *in_degree.get_mut(conn.to_node.as_str()).unwrap() += 1;
        }

        // Find all nodes with no incoming edges
        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| *node)
            .collect();

        let mut result = Vec::new();

        while let Some(node) = queue.pop_front() {
            result.push(node.to_string());

            if let Some(neighbors) = adjacency.get(node) {
                for &neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err(CircuitError::CycleDetected);
        }

        Ok(result)
    }

    /// Get incoming connections for a node
    pub fn get_incoming_connections(&self, node_id: &str) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|conn| conn.to_node == node_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new("test-graph".to_string(), "Test Graph".to_string());
        assert_eq!(graph.id, "test-graph");
        assert_eq!(graph.name, "Test Graph");
        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new("test".to_string(), "Test".to_string());
        let node = Node {
            id: "node1".to_string(),
            block_type: "test".to_string(),
            config: HashMap::new(),
            position: None,
        };
        graph.add_node(node).unwrap();
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_add_connection() {
        let mut graph = Graph::new("test".to_string(), "Test".to_string());

        let node1 = Node {
            id: "node1".to_string(),
            block_type: "test".to_string(),
            config: HashMap::new(),
            position: None,
        };
        let node2 = Node {
            id: "node2".to_string(),
            block_type: "test".to_string(),
            config: HashMap::new(),
            position: None,
        };

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        let conn = Connection {
            from_node: "node1".to_string(),
            from_port: "out".to_string(),
            to_node: "node2".to_string(),
            to_port: "in".to_string(),
        };

        graph.add_connection(conn).unwrap();
        assert_eq!(graph.connections.len(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = Graph::new("test".to_string(), "Test".to_string());

        let node1 = Node {
            id: "node1".to_string(),
            block_type: "test".to_string(),
            config: HashMap::new(),
            position: None,
        };
        let node2 = Node {
            id: "node2".to_string(),
            block_type: "test".to_string(),
            config: HashMap::new(),
            position: None,
        };

        graph.add_node(node1).unwrap();
        graph.add_node(node2).unwrap();

        // Add connection from node1 to node2
        let conn1 = Connection {
            from_node: "node1".to_string(),
            from_port: "out".to_string(),
            to_node: "node2".to_string(),
            to_port: "in".to_string(),
        };
        graph.add_connection(conn1).unwrap();

        // Try to add connection from node2 to node1 (creates cycle)
        let conn2 = Connection {
            from_node: "node2".to_string(),
            from_port: "out".to_string(),
            to_node: "node1".to_string(),
            to_port: "in".to_string(),
        };

        let result = graph.add_connection(conn2);
        assert!(result.is_err());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = Graph::new("test".to_string(), "Test".to_string());

        for i in 1..=3 {
            let node = Node {
                id: format!("node{}", i),
                block_type: "test".to_string(),
                config: HashMap::new(),
                position: None,
            };
            graph.add_node(node).unwrap();
        }

        // node1 -> node2 -> node3
        graph
            .add_connection(Connection {
                from_node: "node1".to_string(),
                from_port: "out".to_string(),
                to_node: "node2".to_string(),
                to_port: "in".to_string(),
            })
            .unwrap();

        graph
            .add_connection(Connection {
                from_node: "node2".to_string(),
                from_port: "out".to_string(),
                to_node: "node3".to_string(),
                to_port: "in".to_string(),
            })
            .unwrap();

        let order = graph.topological_sort().unwrap();
        assert_eq!(order.len(), 3);

        // node1 should come before node2
        let node1_pos = order.iter().position(|n| n == "node1").unwrap();
        let node2_pos = order.iter().position(|n| n == "node2").unwrap();
        assert!(node1_pos < node2_pos);

        // node2 should come before node3
        let node3_pos = order.iter().position(|n| n == "node3").unwrap();
        assert!(node2_pos < node3_pos);
    }
}
