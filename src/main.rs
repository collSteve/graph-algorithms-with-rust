use crate::graph::{BaseVertex, Graph, UnweightedEdge};
mod graph;
mod graph_direct_api;

fn main() {
    let mut my_graph = graph::BaseGraph::<BaseVertex<i32>, UnweightedEdge>::new();

    graph_direct_api::add_vertex(&mut my_graph, 1, 1);
    graph_direct_api::add_vertex(&mut my_graph, 2, 2);
    graph_direct_api::add_vertex(&mut my_graph, 3, 3);
    graph_direct_api::add_vertex(&mut my_graph, 4, 4);
    graph_direct_api::add_vertex(&mut my_graph, 4, 4);

    graph_direct_api::add_edge(&mut my_graph, 1, 1, 2);
    graph_direct_api::add_edge(&mut my_graph, 2, 1, 3);
    graph_direct_api::add_edge(&mut my_graph, 3, 3, 4);

    graph_direct_api::display_neighbors(&my_graph, 1);
}
