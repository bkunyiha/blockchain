use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::RwLock;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Node {
    addr: SocketAddr,
}

impl Node {
    fn new(addr: SocketAddr) -> Node {
        Node { addr }
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn parse_socket_addr(&self) -> SocketAddr {
        self.addr.to_string().parse().unwrap()
    }
}

pub struct Nodes {
    inner: RwLock<HashSet<Node>>,
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes {
            inner: RwLock::new(HashSet::new()),
        }
    }

    pub fn add_node(&self, addr: SocketAddr) {
        let mut inner = self.inner.write().unwrap();
        inner.insert(Node::new(addr));
    }

    pub fn add_nodes(&self, nodes: Vec<SocketAddr>) {
        let mut inner = self.inner.write().unwrap();
        for node in nodes {
            inner.insert(Node::new(node));
        }
    }

    pub fn evict_node(&self, addr: &SocketAddr) {
        let mut inner = self.inner.write().unwrap();
        inner.remove(&Node::new(*addr));
    }

    pub fn first(&self) -> Option<Node> {
        let inner = self.inner.read().unwrap();
        inner.iter().next().cloned()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        self.inner.read().unwrap().iter().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }

    pub fn node_is_known(&self, addr: &SocketAddr) -> bool {
        let inner = self.inner.read().unwrap();
        inner.iter().any(|x| x.get_addr().eq(addr))
    }
}

/// The `Default` trait is implemented for the `Nodes` struct.
///
/// # Implementation
///
/// The `Default` trait is implemented for the `Nodes` struct.
///
/// This calls the `new` method to create a new `Nodes` instance.
impl Default for Nodes {
    fn default() -> Self {
        Self::new()
    }
}
