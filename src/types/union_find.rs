use std::collections::HashMap;

/// Union-Find (Disjoint Set Union) data structure with Path Compression
/// for efficient union and find operations.
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    /// Creates a new UnionFind structure with n elements
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    /// Finds the representative element (root) of an element
    /// with Path Compression for optimal performance
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Unifies two sets by unioning their roots
    pub fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px != py {
            self.parent[px] = py;
        }
    }

    /// Returns a map from root to cluster size
    pub fn cluster_sizes(&mut self) -> HashMap<usize, usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes
    }

    /// Returns the number of distinct clusters
    #[allow(unused)]
    pub fn cluster_count(&mut self) -> usize {
        self.cluster_sizes().len()
    }

    /// Returns all elements grouped by their cluster
    #[allow(unused)]
    pub fn get_clusters(&mut self) -> Vec<Vec<usize>> {
        let mut clusters: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            clusters.entry(root).or_default().push(i);
        }
        clusters.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = UnionFind::new(5);
        assert_eq!(uf.parent.len(), 5);
    }

    #[test]
    fn test_find_single_element() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(4), 4);
    }

    #[test]
    fn test_union_and_find() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);

        let root0 = uf.find(0);
        let root2 = uf.find(2);
        assert_eq!(root0, root2);
    }

    #[test]
    fn test_cluster_sizes() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);

        let sizes = uf.cluster_sizes();
        assert_eq!(sizes.len(), 3);

        let size_values: Vec<_> = sizes.values().copied().collect();
        assert!(size_values.contains(&3));
        assert!(size_values.contains(&2));
        assert!(size_values.contains(&1));
    }

    #[test]
    fn test_cluster_count() {
        let mut uf = UnionFind::new(6);
        assert_eq!(uf.cluster_count(), 6);

        uf.union(0, 1);
        assert_eq!(uf.cluster_count(), 5);

        uf.union(1, 2);
        assert_eq!(uf.cluster_count(), 4);

        uf.union(3, 4);
        assert_eq!(uf.cluster_count(), 3);
    }

    #[test]
    fn test_get_clusters() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);

        let clusters = uf.get_clusters();
        assert_eq!(clusters.len(), 2);

        let mut found_large_cluster = false;
        let mut found_small_cluster = false;

        for cluster in clusters {
            if cluster.len() == 3 {
                found_large_cluster =
                    cluster.contains(&0) && cluster.contains(&1) && cluster.contains(&2);
            } else if cluster.len() == 2 {
                found_small_cluster = cluster.contains(&3) && cluster.contains(&4);
            }
        }

        assert!(found_large_cluster);
        assert!(found_small_cluster);
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);

        let root = uf.find(0);

        assert_eq!(uf.parent[0], root);
    }

    #[test]
    fn test_union_same_set() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);

        let before = uf.cluster_count();
        uf.union(0, 2);
        let after = uf.cluster_count();

        assert_eq!(before, after);
    }

    #[test]
    fn test_large_union_find() {
        let mut uf = UnionFind::new(1000);

        for i in 1..1000 {
            uf.union(i, i - 1);
        }

        assert_eq!(uf.cluster_count(), 1);

        let sizes = uf.cluster_sizes();
        assert_eq!(sizes.values().next().unwrap(), &1000);
    }
}
