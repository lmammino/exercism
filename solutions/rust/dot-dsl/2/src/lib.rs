pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod attrs {
            macro_rules! impl_with_attrs {
                ($ty:ty, $field:ident) => {
                    impl $ty {
                        pub fn with_attrs<K, V>(self, pairs: &[(K, V)]) -> Self
                        where
                            K: AsRef<str>,
                            V: AsRef<str>,
                        {
                            let mut copied = self.clone();
                            copied.$field = pairs
                                .iter()
                                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                                .collect();
                            copied
                        }

                        pub fn attr(&self, key: &str) -> Option<&str> {
                            self.$field.get(key).map(|s| s.as_str())
                        }
                    }
                };
            }

            pub(crate) use impl_with_attrs;
        }

        pub mod edge {
            use std::collections::HashMap;

            use super::attrs::impl_with_attrs;

            #[derive(Debug, Clone, Default, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new<T: AsRef<str>>(from: T, to: T) -> Self {
                    Self {
                        from: from.as_ref().to_string(),
                        to: to.as_ref().to_string(),
                        ..Default::default()
                    }
                }
            }

            impl_with_attrs!(Edge, attrs);
        }

        pub mod node {
            use std::collections::HashMap;

            use super::attrs::impl_with_attrs;

            #[derive(Debug, Clone, Default, PartialEq)]
            pub struct Node {
                label: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new<T: AsRef<str>>(label: T) -> Self {
                    Self {
                        label: label.as_ref().to_string(),
                        ..Default::default()
                    }
                }

                pub fn label(&self) -> &str {
                    &self.label
                }
            }

            impl_with_attrs!(Node, attrs);
        }
    }

    use graph_items::{attrs::impl_with_attrs, edge::Edge, node::Node};

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            let mut copied = self.clone();
            copied.nodes = nodes.to_owned();
            copied
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            let mut copied = self.clone();
            copied.edges = edges.to_owned();
            copied
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.label() == name)
        }
    }

    impl_with_attrs!(Graph, attrs);
}