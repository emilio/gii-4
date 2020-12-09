use std::collections::HashMap;
use std::error::Error;

/// A node id is just a sequential index in an array.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct NodeId(usize);

/// A set of nodes, represented as a chunked bitfield of 32-bit integers.
#[derive(Debug)]
pub struct NodeSet {
    slots: Vec<u32>,
}

impl NodeSet {
    fn index_for(i: usize) -> usize {
        i / 32
    }

    fn shift_for(i: usize) -> usize {
        i % 32
    }

    pub fn is_empty(&self) -> bool {
        for slot in &self.slots {
            if *slot != 0 {
                return false;
            }
        }
        true
    }

    /// Creates an empty node-set with enough capacity to hold all the nodes in
    /// `input`.
    pub fn new(input: &Input) -> Self {
        let slots = vec![0; Self::index_for(input.node_count())];
        Self { slots }
    }

    pub fn contains(&self, node: NodeId) -> bool {
        // This should optimize to a set of bit manipulations hopefully,
        // otherwise we could do that manually.
        let slot = Self::index_for(node.0);
        self.slots[slot] & (1u32 << Self::shift_for(node.0)) != 0
    }

    pub fn insert(&mut self, node: NodeId) {
        let slot = Self::index_for(node.0);
        self.slots[slot] |= 1u32 << Self::shift_for(node.0);
    }
}

/// A map from nodes to a given type, represented as a Vec<>, zero-indexed and
/// lazily growable.
#[derive(Debug, Clone)]
pub struct NodeMap<T> {
    store: Vec<Option<T>>,
}

impl<T> Default for NodeMap<T> {
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

impl<T> NodeMap<T> {
    /// Get a mutable reference to the entry for a given node.
    pub fn get_mut(&mut self, node: NodeId) -> &mut Option<T> {
        if node.0 >= self.store.len() {
            self.store.resize_with(node.0 + 1, Default::default);
        }
        &mut self.store[node.0]
    }

    /// Get an immutable reference to the entry for a given node.
    pub fn get(&self, node: NodeId) -> Option<&T> {
        self.store.get(node.0)?.as_ref()
    }

    /// Insert a node and return the previous entry if any.
    pub fn insert(&mut self, node: NodeId, value: T) -> Option<T> {
        self.get_mut(node).replace(value)
    }

    /// Remove a node from the map, returning the value if any.
    pub fn remove(&mut self, node: NodeId) -> Option<T> {
        self.store.get_mut(node.0)?.take()
    }

    /// Iterates over all members of the map.
    pub fn iter(&self) -> impl Iterator<Item = (NodeId, &T)> {
        self.store
            .iter()
            .enumerate()
            .filter_map(|(i, v)| Some((NodeId(i), v.as_ref()?)))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Contact {
    // TODO(emilio): Do we need the from edge here?
    pub from: NodeId,
    pub to: NodeId,
    pub k: f32,
    pub distance: Option<f32>,
}

#[derive(Default, Debug)]
struct NodeStore {
    /// Map from node id to node name (zero-indexed by id). We could use NodeMap
    /// here, but since this is the canonical source of stuff there's no
    node_names: Vec<String>,
    /// Map from name to id.
    nodes_by_name: HashMap<String, NodeId>,
}

impl NodeStore {
    fn get_or_insert(&mut self, name: String) -> NodeId {
        use std::collections::hash_map::Entry;
        match self.nodes_by_name.entry(name) {
            Entry::Vacant(v) => {
                let id = NodeId(self.node_names.len());
                self.node_names.push(v.key().clone());
                *v.insert(id)
            }
            Entry::Occupied(e) => *e.get(),
        }
    }

    fn len(&self) -> usize {
        self.node_names.len()
    }
}

/// This effectively represents the edges of our graph, by mapping a given node
/// (zero-indexed-by-id) to its set of contacts.
#[derive(Debug, Default)]
struct Contacts {
    contacts: NodeMap<Vec<Contact>>,
}

impl Contacts {
    fn get_mut(&mut self, node: NodeId) -> &mut Vec<Contact> {
        self.contacts
            .get_mut(node)
            .get_or_insert_with(Default::default)
    }

    fn get(&self, node: NodeId) -> &[Contact] {
        match self.contacts.get(node) {
            Some(c) => &*c,
            None => &[],
        }
    }
}

#[derive(Debug)]
pub struct Input {
    nodes: NodeStore,
    contacts: Contacts,
}

impl Input {
    /// Crappy parser for the pdb subset we need.
    pub fn parse(file: &str) -> Result<Self, Box<dyn Error>> {
        use std::io::{BufRead, BufReader};
        let file = BufReader::new(std::fs::File::open(file)?);

        let mut nodes = NodeStore::default();
        let mut contacts = Contacts::default();
        let mut lineno = 0;
        for line in file.lines() {
            lineno += 1;
            let line = line?;
            let mut pieces = line.split('(');
            match pieces.next() {
                Some(name) => {
                    if name != "contacto" {
                        return Err(
                            format!("Unknown function name {} at line {}", name, lineno).into()
                        );
                    }
                }
                None => continue,
            }
            let args = match pieces.next() {
                Some(v) => v.trim().trim_end_matches('.').trim_end_matches(')'),
                None => return Err(format!("Didn't find closing paren in line {}", lineno).into()),
            };

            let args = args.split(",").map(|arg| arg.trim()).collect::<Vec<_>>();
            if args.len() != 3 && args.len() != 4 {
                return Err(format!(
                    "Expected three or four arguments, got {} at line {}",
                    args.len(),
                    lineno
                )
                .into());
            }

            let first = nodes.get_or_insert(args[0].to_owned());
            let second = nodes.get_or_insert(args[1].to_owned());
            let k = args[2].parse::<f32>()?;
            let distance = match args.get(3) {
                Some(d) => Some(d.parse::<f32>()?),
                None => None,
            };

            contacts.get_mut(first).push(Contact {
                from: first,
                to: second,
                k,
                distance,
            });

            contacts.get_mut(second).push(Contact {
                from: second,
                to: first,
                k,
                distance,
            });
        }

        Ok(Self { nodes, contacts })
    }

    pub fn node_name(&self, node: NodeId) -> &str {
        &self.nodes.node_names[node.0]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeId> {
        (0..self.node_count()).map(NodeId)
    }

    pub fn node_by_name(&self, name: &str) -> Option<NodeId> {
        self.nodes.nodes_by_name.get(name).cloned()
    }

    pub fn edges_from(&self, node: NodeId) -> &[Contact] {
        self.contacts.get(node)
    }

    pub fn dump(&self) {
        for node in self.nodes() {
            println!("Edges from {} ({:?})", self.node_name(node), node);
            for edge in self.edges_from(node) {
                println!(
                    "  -> {} ({:?}) with k={}, distance={:?}",
                    self.node_name(edge.to),
                    edge.to,
                    edge.k,
                    edge.distance
                );
            }
        }
    }
}
