struct WeightedDirectedGraph<Cost> {
    n: usize,
    g: Vec<Vec<(usize, Cost)>>,
}