use crate::graph::{Edge, EdgeId, Graph, Vertex, VertexId};

pub fn display_node_edge<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(
    graph: &TG,
) {
    println!("Vertex count: {}", graph.get_vertex_count());
    println!("Edge count: {}", graph.get_edge_count());

    for node in graph.get_all_vertices() {
        println!("Node: id:{}, data:{}", node.get_id(), node.get_data());
    }

    for edge in graph.get_all_edges().iter() {
        println!(
            "Edge: id:{}, source:{}, target:{}",
            edge.get_id(),
            edge.get_source(),
            edge.get_target()
        );
    }
}

pub fn add_edge<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(
    graph: &mut TG,
    edge_id: EdgeId,
    source_id: VertexId,
    target_id: VertexId,
) {
    match graph.add_edge(edge_id, source_id, target_id) {
        Ok(()) => {
            println!("Edge added");
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

pub fn add_vertex<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(
    graph: &mut TG,
    vertex_id: VertexId,
    data: TD,
) {
    match graph.add_vertex(vertex_id, data) {
        Ok(()) => {
            println!("Vertex added");
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

pub fn display_neighbors<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(
    graph: &TG,
    vertex_id: VertexId,
) {
    let neighbor_ids = graph.get_neighbors(vertex_id);

    println!(
        "Neighbors of vertex {} has {} neighbors:",
        vertex_id,
        neighbor_ids.len()
    );
    for id in neighbor_ids {
        match graph.get_vertex(id) {
            Ok(vertex) => {
                println!("id:{}, data:{}", vertex.get_id(), vertex.get_data());
            }
            Err(error) => {
                println!("Weird Error: {}", error);
            }
        }
    }
}
