use std::cmp::Ordering;
use std::fmt;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge<T> {
    pub source: T,
    pub target: T,
    pub weight: OrderedFloat<f64>,
}

impl<T: fmt::Debug + Ord + Clone> Edge<T> {
    pub fn new(source: T, target: T, weight: f64) -> Self {
        Edge { source, target, weight: OrderedFloat(weight) }
    }

    pub fn inverted(&self) -> Self {
        Edge {
            source: self.target.clone(),
            target: self.source.clone(),
            weight: self.weight,
        }
    }
}

impl<T: fmt::Debug> fmt::Display for Edge<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\t{:?}\t{}", self.source, self.target, self.weight.0 * 1000.0)
    }
}

impl<T: Ord> PartialOrd for Edge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Edge<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight
            .cmp(&other.weight)
            .then_with(|| self.source.cmp(&other.source))
            .then_with(|| self.target.cmp(&other.target))
    }
}

// Undirected Edge
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UndirectedEdge<T> {
    pub source: T,
    pub target: T,
    pub weight: OrderedFloat<f64>,
}

impl<T: Ord + Clone> UndirectedEdge<T> {
    pub fn new(source: T, target: T, weight: f64) -> Self {
        let (s, t) = if source > target {
            (target, source)
        } else {
            (source, target)
        };
        UndirectedEdge {
            source: s,
            target: t,
            weight: OrderedFloat(weight),
        }
    }

    pub fn inverted(&self) -> Self {
        self.clone() // No change needed, undirected edges are symmetric
    }
}