///! This trait represents an edge
pub trait Edge<V> {
    fn source(&self) -> &V;
    fn target(&self) -> &V;
}

///! Register this trait for a simple edge tuple
impl<V> Edge<V> for (V, V) {
    fn source(&self) -> &V {
        &self.0
    }
    fn target(&self) -> &V {
        &self.1
    }
}

///! My own representation of a graph
pub struct Graph<V, E = (V, V)>
where
    V: std::cmp::PartialEq,
    E: std::cmp::PartialEq + Edge<V>,
{
    ///! Vertices
    v: Vec<V>,
    ///! Edges
    e: Vec<E>,
    ///! Oriented
    o: bool,
}

#[derive(PartialEq, Debug)]
///! Any operation on the graph generates an "error"
pub enum GraphError {
    Unknown = -1,
    Ok = 0,

    VertexAlreadyExists,
    InvalidVertex,

    EdgeAlreadyExists,
    InvalidEdge,
    InvalidEdgeVertices,
}

impl<V, E> Graph<V, E>
where
    V: std::cmp::PartialEq,
    E: std::cmp::PartialEq + Edge<V>,
{
    pub fn new(o: bool) -> Graph<V, E> {
        Graph {
            v: Vec::new(),
            e: Vec::new(),
            o: o,
        }
    }

    pub fn vertex_exists(&self, v: &V) -> bool {
        self.v.contains(v)
    }

    pub fn edge_exists(&self, e: &E) -> bool {
        self.e.iter().any(|edge| {
            edge.source() == e.source() && edge.target() == e.target()
                || (!self.o && edge.source() == e.target() && edge.target() == e.source())
        })
    }

    pub fn is_neighbor(&self, v1: &V, v2: &V) -> bool {
        self.e.iter().any(|e| {
            (e.source() == v1 && e.target() == v2)
                || (!self.o && e.source() == v2 && e.target() == v1)
        })
    }

    pub fn neighbors(&self, v: &V) -> Vec<&V> {
        self.e
            .iter()
            .filter_map(|e| {
                if e.source() == v {
                    Some(e.target())
                } else if !self.o && e.target() == v {
                    Some(e.source())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn neighbors_iter<'a>(&'a self, v: &'a V) -> impl Iterator<Item = &'a V> {
        self.e.iter().filter_map(move |e| {
            if e.source() == v {
                Some(e.target())
            } else if !self.o && e.target() == v {
                Some(e.source())
            } else {
                None
            }
        })
    }

    pub fn degree(&self, v: &V) -> usize {
        self.neighbors(v).len()
    }

    pub fn isolated(&self, v: &V) -> bool {
        self.degree(v) == 0
    }

    pub fn add_vertex(&mut self, v: V) -> GraphError {
        if self.vertex_exists(&v) {
            return GraphError::VertexAlreadyExists;
        } else {
            self.v.push(v);
            return GraphError::Ok;
        }
    }

    pub fn add_edge(&mut self, e: E) -> GraphError {
        if self.edge_exists(&e) {
            return GraphError::EdgeAlreadyExists;
        } else if self.vertex_exists(e.source()) && self.vertex_exists(e.target()) {
            self.e.push(e);
            return GraphError::Ok;
        } else {
            return GraphError::InvalidEdgeVertices;
        }
    }

    pub fn order(&self) -> usize {
        self.v.len()
    }

    pub fn length(&self) -> usize {
        self.e.len()
    }

    ///! Graph Union
    fn extend(&mut self, rhs: Self) {
        assert!(self.o == rhs.o);
        for v in rhs.v {
            if !self.vertex_exists(&v) {
                self.v.push(v);
            }
        }

        for e in rhs.e {
            if !self.edge_exists(&e) {
                self.e.push(e);
            }
        }
    }

    fn remove_edge(&mut self, e: &E) -> GraphError {
        if let Some(idx) = self.e.iter().position(|edge| {
            edge.source() == e.source() && edge.target() == e.target()
                || (!self.o && edge.source() == e.target() && edge.target() == e.source())
        }) {
            self.e.remove(idx);
            GraphError::Ok
        } else {
            GraphError::InvalidEdge
        }
    }

    fn remove_edges_incident_to(&mut self, v: &V) -> GraphError {
        if !self.vertex_exists(&v) {
            return GraphError::InvalidVertex;
        } else {
            self.e
                .retain(|edge| edge.source() != v && edge.target() != v);
            return GraphError::Ok;
        }
    }

    fn remove_vertex(&mut self, v: V) -> GraphError {
        if !self.vertex_exists(&v) {
            return GraphError::InvalidVertex;
        } else {
            if self.degree(&v) == 0 {
                if let Some(idx) = self.v.iter().position(|vertex| vertex == &v) {
                    self.v.remove(idx);
                }
            } else {
                self.remove_edges_incident_to(&v);
                if let Some(idx) = self.v.iter().position(|vertex| vertex == &v) {
                    self.v.remove(idx);
                }
            }

            return GraphError::Ok;
        }
    }
}

///! Allow comparaisons between graphs
impl<V, E> std::cmp::PartialEq for Graph<V, E>
where
    V: std::cmp::PartialEq,
    E: std::cmp::PartialEq + Edge<V>,
{
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.e == other.e
    }
}

///! Additional Eq properties (car ssi)
impl<V, E> Eq for Graph<V, E>
where
    V: Eq,
    E: Eq + Edge<V>,
{
}

impl<V, E> std::ops::Add<Self> for Graph<V, E>
where
    V: PartialEq,
    E: PartialEq + Edge<V>,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.extend(rhs);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let mut graph = Graph::<u32>::new(true);

        graph.add_vertex(1);
        assert!(graph.vertex_exists(&1));
        assert!(!graph.vertex_exists(&2));

        graph.add_vertex(2);
        graph.add_edge((1, 2));
        assert!(graph.edge_exists(&(1, 2)));
        assert!(!graph.edge_exists(&(2, 1)));

        graph.remove_vertex(1);
        assert!(!graph.vertex_exists(&1));
        assert!(!graph.edge_exists(&(1, 2)));

        assert_eq!(graph.remove_edge(&(1,2)), GraphError::InvalidEdge);

        graph = Graph::<u32>::new(false);

        graph.add_vertex(1);
        assert!(graph.vertex_exists(&1));
        assert!(!graph.vertex_exists(&2));

        graph.add_vertex(2);
        graph.add_edge((1, 2));
        assert!(graph.edge_exists(&(1, 2)));
        assert!(graph.edge_exists(&(2, 1))); // Non oriented, so this should pass

        graph.remove_vertex(1);
        assert!(!graph.vertex_exists(&1));
        assert!(!graph.edge_exists(&(1, 2)));
        assert!(!graph.edge_exists(&(2, 1)));

        assert_eq!(graph.remove_edge(&(1,2)), GraphError::InvalidEdge);

    }
}
