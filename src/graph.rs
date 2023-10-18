pub mod Graph {

    pub type VertexId = i32;
    pub type EdgeId = i32;

    pub struct BaseVertex<TData> {
        id: VertexId,
        data: TData,
    }

    pub struct UnweightedEdge {
        id: EdgeId,
        source: i32,
        target: i32,
    }

    impl Edge for UnweightedEdge {
        fn new(id: EdgeId, source: VertexId, target: VertexId) -> Self {
            return UnweightedEdge {
                id: id,
                source: source,
                target: target,
            }
        }
        fn get_id(&self) -> EdgeId {
            return self.id;
        }

        fn get_source(&self) -> VertexId {
            return self.source;
        }

        fn get_target(&self) -> VertexId {
            return self.target;
        }
    }
    

    pub struct WeightedEdge {
        id: EdgeId,
        source: i32,
        target: i32,
        weight: f32,
    }

    pub trait Edge {
        fn new(id: EdgeId, source: VertexId, target: VertexId) -> Self;
        fn get_id(&self) -> EdgeId;
        fn get_source(&self) -> VertexId;
        fn get_target(&self) -> VertexId;
    }

    pub trait Vertex<TData> {
        fn new(id: VertexId, data: TData) -> Self;
        fn get_id(&self) -> VertexId;
        fn get_data(&self) -> &TData;
    }

    pub trait Graph<TVertex: Vertex<TData>, TEdge: Edge, TData> {
        fn new() -> Self;
        fn add_vertex(&mut self, id: VertexId, data: TData) -> Result<(), GraphError>;
        fn add_edge(&mut self, id: EdgeId, source: VertexId, target: VertexId) -> Result<(), GraphError>;
        
        fn get_vertex_count(&self) -> i32;
        fn get_edge_count(&self) -> i32;
        fn get_vertex_data(&self, id: VertexId) -> Result<&TData, GraphError>;

        fn get_all_vertices(&self) -> &Vec<TVertex>;
        fn get_all_edges(&self) -> &Vec<TEdge>;

        fn get_neighbors(&self, id: VertexId) -> Vec<VertexId> {
            let mut neighbors: Vec<VertexId> = Vec::new();

            for edge in self.get_all_edges() {
                if edge.get_source() == id {
                    neighbors.push(edge.get_target());
                }
            }

            return neighbors;
        }

        fn get_vertex(&self, id: VertexId) -> Result<&TVertex, GraphError> {
            let node_option = self.get_all_vertices().iter().find(|&node| node.get_id() == id);
            match node_option {
                Some(node) => return Ok(node),
                None => return Err(GraphError::VertexNotFound),
            }
        }

        fn get_edge(&self, id: VertexId) -> Result<&TEdge, GraphError> {
            let edge_option = self.get_all_edges().iter().find(|&edge| edge.get_id() == id);
            match edge_option {
                Some(edge) => return Ok(edge),
                None => return Err(GraphError::EdgeNotFound)
            }
        }
    }

    pub struct BaseGraph<TVertex, TEdge> {
        nodes: Vec<TVertex>,
        edges: Vec<TEdge>,
    }

    impl<TData> Vertex<TData> for BaseVertex<TData> {
        fn new(id: VertexId, data: TData) -> Self {
            BaseVertex {
                id: id,
                data: data,
            }
        }

        fn get_id(&self) -> VertexId {
            self.id
        }

        fn get_data(&self) -> &TData {
            return &self.data;
        }
    }

    pub enum GraphError {
        VertexNotFound,
        EdgeNotFound,
        EdgeIdAlreadyExist(EdgeId),
        VertexIdAlreadyExist(EdgeId),
    }

    impl std::fmt::Display for GraphError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GraphError::VertexNotFound => write!(f, "Vertex not found"),
                GraphError::EdgeNotFound => write!(f, "Edge not found"),
                GraphError::EdgeIdAlreadyExist(id) => write!(f, "Edge id <{}> already exist", id),
                GraphError::VertexIdAlreadyExist(id) => write!(f, "Vertex id <{}> already exist", id),
            }
        }
    }

    impl<TData, TVertex: Vertex<TData>, TEdge: Edge> Graph<TVertex, TEdge, TData> for BaseGraph<TVertex, TEdge> {

        fn get_vertex_count(&self) -> i32 {
            return self.nodes.len() as i32;
        }

        fn get_edge_count(&self) -> i32 {
            return self.edges.len() as i32;
        }

        fn get_vertex_data(&self, id: VertexId) -> Result<&TData, GraphError> {
            let node_result = self.get_vertex(id);
            match node_result {
                Ok(node) => return Ok(node.get_data()),
                Err(_) => return Err(GraphError::VertexNotFound),
            }
        }

        fn new() -> Self {
            BaseGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            }
        }

        fn add_vertex(&mut self, id: VertexId, data: TData) -> Result<(), GraphError> {
            let id_exist = self.nodes.iter().any(|n|n.get_id() == id);

            if id_exist {
                return Err(GraphError::VertexIdAlreadyExist(id));
            }
            self.nodes.push(TVertex::new(id, data));
            return Ok(());
        }

        fn add_edge(&mut self, id: EdgeId, source: VertexId, target: VertexId) -> Result<(), GraphError> {
            let source_exist = self.nodes.iter().any(|n|n.get_id() == source);
            let target_exist = self.nodes.iter().any(|n|n.get_id() == target);

            let id_exist = self.edges.iter().any(|n|n.get_id() == id);
            if id_exist {
                return Err(GraphError::EdgeIdAlreadyExist(id));
            }

            if source_exist && target_exist {
                self.edges.push(TEdge::new(id, source, target));
                return Ok(());
            } else {
                return Err(GraphError::VertexNotFound);
            }
        }

        fn get_all_edges(&self) -> &Vec<TEdge> {
            return &self.edges;
        }

        fn get_all_vertices(&self) -> &Vec<TVertex> {
            return &self.nodes;
        }
    }
}