use graph::Graph::{EdgeId, VertexId};

use crate::graph::Graph::{Graph, Vertex, BaseGraph, BaseVertex, Edge, UnweightedEdge};

mod graph;

fn main() {
    let mut my_graph = graph::Graph::BaseGraph::<BaseVertex<i32>, UnweightedEdge>::new();

    add_vertex(&mut my_graph, 1, 1);
    add_vertex(&mut my_graph, 2, 2);
    add_vertex(&mut my_graph, 3, 3);
    add_vertex(&mut my_graph, 4, 4);
    add_vertex(&mut my_graph, 4, 4);

    add_edge(&mut my_graph, 1, 1, 2);
    add_edge(&mut my_graph, 2, 1, 3);
    add_edge(&mut my_graph, 3, 3, 4);

    display_neighbors(&my_graph, 1);
}

fn display_node_edge<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(graph: &TG)  {
    println!("Vertex count: {}", graph.get_vertex_count());
    println!("Edge count: {}", graph.get_edge_count());

    for node in graph.get_all_vertices() {
        println!("Node: id:{}, data:{}", node.get_id(), node.get_data());
    }

    for edge in graph.get_all_edges().iter() {
        println!("Edge: id:{}, source:{}, target:{}", edge.get_id(), edge.get_source(), edge.get_target());
    }
}

fn add_edge<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(graph: &mut TG, edge_id: EdgeId, source_id: VertexId, target_id: VertexId) {
    match graph.add_edge(edge_id, source_id, target_id) {
        Ok(()) => {
            println!("Edge added");
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn add_vertex<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(graph: &mut TG, vertex_id: VertexId, data: TD) {
    match graph.add_vertex(vertex_id, data) {
        Ok(()) => {
            println!("Vertex added");
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn display_neighbors<TG: Graph<TV, TE, TD>, TV: Vertex<TD>, TE: Edge, TD: std::fmt::Display>(graph: &TG, vertex_id: VertexId) {
    let neighbor_ids = graph.get_neighbors(vertex_id);

    println!("Neighbors of vertex {} has {} neighbors:", vertex_id, neighbor_ids.len());
    for id in neighbor_ids {
        match graph.get_vertex(id) {
            Ok(vertex) => {
                println!("id:{}, data:{}", vertex.get_id(), vertex.get_data());
            },
            Err(error) => {
                println!("Weird Error: {}", error);
            }
        }
    }
}
