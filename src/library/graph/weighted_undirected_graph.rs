struct WeightedUndirectedGraph<Cost> {
    n: usize,
    g: Vec<Vec<(usize, Cost)>>,
}