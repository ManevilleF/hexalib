use crate::Hex;
use std::collections::{BinaryHeap, HashMap};

struct Node {
    coord: Hex,
    cost: u32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        rhs.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.cost.cmp(&self.cost)
    }
}

fn reconstruct_path(came_from: &HashMap<Hex, Hex>, end: Hex) -> Vec<Hex> {
    let mut path: Vec<_> =
        std::iter::successors(Some(end), move |&current| came_from.get(&current).copied())
            .collect();
    path.reverse();
    path
}

/// Performs A star pathfinding between `start` and `end`.
/// The `cost` parameter should give the cost of each coordinate (`Some`) or indicate the
/// coordinate is not included in the pathfinding (`None`).
/// This function already takes care of heuristics based on the distance between `start` and `end`.
///
/// # Examples
///
/// - Compute a A star with no boundaries and some forbidden tiles
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashSet;
/// use hexx::algorithms::a_star;
///
/// let start = hex(0, 0);
/// let end = hex(10, 0);
/// let forbidden_coords: HashSet<Hex> = HashSet::new();
/// // Add forbidden coordinates
/// // forbidden_coords.insert(hex(2, 0));
/// // ..
/// let path = a_star(start, end, |h| (!forbidden_coords.contains(&h)).then_some(0));
/// ```
/// - Compute a A star with no boundaries and some biome costs
///
/// ```rust
/// # use hexx::*;
/// # use std::collections::HashMap;
/// use hexx::algorithms::a_star;
///
/// enum Biome {
///    Mountain,
///    Plains,
///    Forest,
///    Desert
/// }
///
/// impl Biome {
///
///    pub fn cost(&self) -> Option<u32> {
///       match self {
///          Self::Mountain => None, // Moutains are not included in pathfinding
///          Self::Plains => Some(0),
///          Self::Forest => Some(1),
///          Self::Desert => Some(2)
///       }
///    }
/// }
///
/// let start = hex(0, 0);
/// let end = hex(10, 0);
/// let mut biomes: HashMap<Hex, Biome> = HashMap::new();
/// // Set coordinate biomes
/// // biomes.insert(hex(1, 2), Biome::Mountain);
/// // ..
/// let path = a_star(start, end, |h| biomes.get(&h).and_then(|b| b.cost()));
/// ```
pub fn a_star(start: Hex, end: Hex, cost: impl Fn(Hex) -> Option<u32>) -> Option<Vec<Hex>> {
    let heuristic = |h: Hex| h.unsigned_distance_to(start);

    let start_node = Node {
        coord: start,
        cost: heuristic(start),
    };
    let mut open = BinaryHeap::new();
    let mut costs = HashMap::new();
    costs.insert(start, start_node.cost);
    let mut came_from = HashMap::new();
    open.push(start_node);

    while let Some(node) = open.pop() {
        if node.coord == end {
            return Some(reconstruct_path(&came_from, end));
        }
        for neighbor in node.coord.all_neighbors() {
            let Some(cost) = cost(neighbor) else { continue };
            let neighbor_cost = costs[&node.coord] + cost + heuristic(neighbor);
            if !costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost {
                came_from.insert(neighbor, node.coord);
                costs.insert(neighbor, neighbor_cost);
                open.push(Node {
                    coord: neighbor,
                    cost: neighbor_cost,
                });
            }
        }
    }
    None
}
