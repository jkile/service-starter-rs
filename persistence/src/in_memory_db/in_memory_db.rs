// use std::{
//     cell::RefCell,
//     collections::{BTreeMap, HashMap},
//     hash::Hash,
//     sync::Arc,
// };

// use tokio::sync::Mutex;

// #[derive(Debug, Clone)]
// pub struct InMemoryDb<K, V>
// where
//     K: Hash + Eq,
// {
//     pub store: Arc<Mutex<RefCell<HashMap<TableNames, Table<K, V>>>>>,
// }

// #[derive(Debug)]
// struct Table<K, V>
// where
//     K: Hash + Eq,
// {
//     pub rows: RefCell<HashMap<K, V>>,
// }

// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub enum TableNames {
//     Users,
//     Sessions,
// }
