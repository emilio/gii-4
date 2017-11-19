#[macro_use]
extern crate clap;

use std::{cmp, hash, io};
use std::rc::Rc;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::path::Path;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct NodeId(Rc<String>);

impl NodeId {
    pub fn new<T: Into<String>>(name: T) -> Self {
        NodeId(Rc::new(name.into()))
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct ItemId(Rc<String>);

impl ItemId {
    pub fn new<T: Into<String>>(name: T) -> Self {
        ItemId(Rc::new(name.into()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Edge {
    to: NodeId,
    distance: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Order {
    items: HashMap<ItemId, u32>,
    // We need to keep track of the nodes we've taken stuff from, to avoid
    // gathering items from the same storage twice.
    nodes_visited: HashSet<NodeId>,
    items_left: u32,
}

impl Order {
    pub fn from_file(path: &Path) -> Result<Self, io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = BufReader::new(File::open(path)?);
        let mut order_items = HashMap::default();
        let mut total_items = 0;

        for (i, line) in file.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('%') {
                continue;
            }

            let mut items = line.split(',');
            let item = ItemId::new(items.next().unwrap());
            let amount = match items.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(i) => i,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Line {} didn't contain a valid amount: {:?}", i, line),
                    ));
                }
            };

            if amount == 0 {
                continue;
            }

            total_items += amount;
            *order_items.entry(item).or_insert(0) += amount;
        }

        Ok(Order {
            items: order_items,
            items_left: total_items,
            nodes_visited: HashSet::default(),
        })
    }

    /// Substract the pending items for the order, assuming we visit `node_id`.
    fn gather_items(&mut self, map: &Map, node_id: &NodeId) {
        if self.items_left == 0 {
            return;
        }

        let item_storage = match map.storage.get(node_id) {
            Some(s) => s,
            None => return,
        };

        if !self.nodes_visited.insert(node_id.clone()) {
            // We had already taken stuff from this node.
            return;
        }

        let items = &mut self.items;
        let items_left = &mut self.items_left;
        // We assume that we only visit each node once, which is ok to find the
        // optimal path.
        items.retain(|item, value| {
            debug_assert_ne!(*value, 0);

            let amount_in_storage = match item_storage.get(item) {
                Some(amount) => *amount,
                None => return true,
            };

            let amount_to_pick = cmp::min(*value, amount_in_storage);
            *value -= amount_to_pick;

            debug_assert!(
                *items_left >= amount_to_pick,
                "Ran out of items when grabbing {:?}? {:?} >= {:?}",
                item,
                items_left,
                amount_to_pick,
            );

            *items_left -= amount_to_pick;

            *value != 0
        });
    }
}

#[derive(Debug, Default)]
struct Map {
    /// A map from an node to all the paths that you can take from it.
    graph: HashMap<NodeId, Vec<Edge>>,
    /// A map from an node to the storage, which can contain items, and for each
    /// item the amount of items of that kind that exist.
    storage: HashMap<NodeId, HashMap<ItemId, u32>>,
    /// The total stock, used to compute whether we can fulfill the order in the
    /// first place.
    stock: HashMap<ItemId, u32>,
}

/// A node in the graph is the position we're in and the amount of items we've
/// already gathered. If `partial_order` is empty, then we've completed our
/// search of items, and we should only find the fastest way to the end node.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    id: NodeId,
    partial_order: Order,
    distance_walked_so_far: u32,
}

impl hash::Hash for Node {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.id.hash(s)
    }
}

impl PartialOrd<Node> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(other.heuristic_score().cmp(&self.heuristic_score()))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Node {
    fn heuristic_score(&self) -> u32 {
        self.distance_walked_so_far + self.partial_order.items_left
    }

    /// Transition from one map to another, taking all the available items to
    /// fulfill the order with it.
    fn transition(map: &Map, from_node: &Node, to_node: &NodeId, distance: u32) -> Node {
        // NOTE(emilio): This assumes that we can never get back to a node we've
        // visited
        let mut new_order = from_node.partial_order.clone();
        new_order.gather_items(map, to_node);

        Node {
            id: to_node.clone(),
            partial_order: new_order,
            distance_walked_so_far: from_node.distance_walked_so_far + distance,
        }
    }
}

impl Map {
    pub fn from_file(file_name: &Path) -> Result<Self, io::Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = BufReader::new(File::open(file_name)?);
        let mut map = Map::default();

        for (i, line) in file.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('%') {
                continue;
            }

            // Pretty crappy parser for what we need.
            let opening_paren = match line.find('(') {
                Some(p) => p,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Line {} wasn't a function call: {:?}", i, line),
                    ));
                }
            };

            let closing_paren = match line[opening_paren..].find(')') {
                Some(p) => opening_paren + p,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Line {} wasn't a function call: {:?}", i, line),
                    ));
                }
            };

            let args = line[opening_paren + 1..closing_paren]
                .split(',')
                .map(|s| s.trim())
                .collect::<Vec<_>>();

            if args.len() != 3 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Expected exactly 3 arguments at line {}, got: {:?}",
                        i,
                        args,
                    ),
                ));
            }

            let third_arg = match args[2].parse::<u32>() {
                Ok(a) => a,
                Err(parse_error) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        parse_error
                    ));
                }
            };

            let function_name = &line[..opening_paren];
            match function_name {
                "conectado" => {
                    let node_from = NodeId::new(args[0]);
                    let node_to = NodeId::new(args[1]);
                    let distance = third_arg;

                    if node_from == node_to {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Self-referencing nodes in line {}: {:?}",
                                i,
                                node_from,
                            )
                        ));
                    }

                    // Edges are bi-directional.
                    map.graph.entry(node_from.clone())
                        .or_insert_with(Vec::new)
                        .push(Edge { to: node_to.clone(), distance });

                    map.graph.entry(node_to)
                        .or_insert_with(Vec::new)
                        .push(Edge { to: node_from, distance });
                },
                "ubicacion" => {
                    let item = ItemId::new(args[0]);
                    let node = NodeId::new(args[1]);
                    let amount = third_arg;
                    *map.stock
                        .entry(item.clone())
                        .or_insert(0) += amount;
                    *map.storage
                        .entry(node)
                        .or_insert_with(HashMap::default)
                        .entry(item)
                        .or_insert(0) += amount;
                },
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Unknown function name at line {}: {:?}",
                            i,
                            function_name,
                        )
                    ));
                }
            };
        }

        Ok(map)
    }

    fn can_fullfill(&self, order: &Order) -> bool {
        for (item, amount) in order.items.iter() {
            let amount_in_stock = match self.stock.get(&item) {
                None => return false,
                Some(stock_amount) => *stock_amount,
            };

            if *amount > amount_in_stock {
                return false;
            }
        }

        true
    }

    fn solve(
        &self,
        start: &NodeId,
        end: &NodeId,
        order: &Order,
    ) -> Result<Vec<Node>, ()> {
        use std::collections::hash_map::Entry;

        if !self.can_fullfill(order) {
            return Err(());
        }

        // Now we know there's a solution.
        let mut visited = HashSet::<Node>::default();

        let start_node = {
            let mut order = order.clone();
            order.gather_items(self, start);

            Node {
                id: start.clone(),
                partial_order: order,
                distance_walked_so_far: 0,
            }
        };

        let mut score = HashMap::<Node, u32>::new();
        score.insert(start_node.clone(), 0);

        let mut unevaluated_nodes = {
            let mut set = BinaryHeap::new();
            set.push(start_node);
            set
        };

        let mut came_from = HashMap::<Node, Node>::new();

        while let Some(node) = unevaluated_nodes.pop() {
            if node.id == *end && node.partial_order.items_left == 0 {
                // Found the path, we're done!
                let mut path = vec![];
                let mut current = node;
                loop {
                    match came_from.remove(&current) {
                        None => {
                            path.push(current);
                            path.reverse();
                            return Ok(path);
                        }
                        Some(other_node) => {
                            path.push(current);
                            current = other_node;
                        }
                    }
                }
            }

            visited.insert(node.clone());
            let neighbours = match self.graph.get(&node.id) {
                Some(neighbours) => &*neighbours,
                None => continue,
            };

            let current_score = score[&node];
            for neighbour in neighbours {
                let Edge { ref to, distance } = *neighbour;

                let neighbor_node = Node::transition(self, &node, to, distance);
                if visited.contains(&neighbor_node) {
                    continue;
                }

                unevaluated_nodes.push(neighbor_node.clone());

                let tentative_score = current_score + distance;
                match score.entry(neighbor_node.clone()) {
                    Entry::Occupied(ref mut occupied_entry) => {
                        if tentative_score >= *occupied_entry.get() {
                            continue;
                        }
                        occupied_entry.insert(tentative_score);
                    }
                    Entry::Vacant(mut vacant_entry) => {
                        vacant_entry.insert(tentative_score);
                    }
                }

                came_from.insert(neighbor_node, node.clone());
            }
        }

        return Err(());
    }
}

fn main() {
    let matches = app_from_crate!()
        .args_from_usage(
            "<map>               'File where the prolog statements that define \
                                  the map are contained'

             <order>             'File where the ordered items and amounts are \
                                  contained'

             -s, --start=[start] 'The start node, defaults to \"S\"'
             -e, --end=[end]     'The end node, defaults to \"T\"'",
        )
        .get_matches();

    let map = Path::new(matches.value_of("map").unwrap());
    let order = Path::new(matches.value_of("order").unwrap());
    let start = NodeId::new(matches.value_of("start").unwrap_or("S"));
    let end = NodeId::new(matches.value_of("end").unwrap_or("T"));

    let map = Map::from_file(&map).expect("Couldn't read map");
    let order = Order::from_file(&order).expect("Couldn't read order");

    // println!("{:?}", map);

    let optimal_path = map.solve(&start, &end, &order);

    println!("{:?}\n\n", optimal_path);

    let mut last_items_left = order.items_left;
    println!("Initial order: {:?}", order);
    if let Ok(optimal_path) = optimal_path {
        for item in &optimal_path {
            println!("Visited: {:?}", item.id);
            if item.partial_order.items_left != last_items_left {
                println!(" > Order changed to: {:?}", item.partial_order);
            }
            last_items_left = item.partial_order.items_left;
        }
    }
}
