use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::nodes_file::NodesFile;

//const D: usize = 3;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub parent_node_id: Option<u64>,
    pub keys: Vec<u64>,
    pub children: Vec<Option<u64>>,
    pub is_leaf: bool,
    pub id: u64,
}

pub enum Direction {
    Smaller,
    Larger,
}

impl Node {
    fn new(is_leaf: bool, id: u64, order: usize) -> Self {
        Self {
            parent_node_id: None,
            keys: Vec::with_capacity(order * 2 + 1),
            children: Vec::with_capacity(order * 2 + 2),
            is_leaf,
            id,
        }
    }

    pub fn search(&self, btree: &mut BTree, key: u64) -> Result<(u64, u64), (u64, u64)> {
        match self.keys.binary_search(&key) {
            Ok(i) => return Ok((i as u64, self.id)),
            Err(i) => match self.children[i] {
                Some(child_id) => {
                    let child = btree.nodes_file.get_node(child_id);
                    return child.search(btree, key);
                }
                None => return Err((i as u64, self.id)),
            },
        }
    }

    fn basic_insert(
        &mut self,
        btree: &mut BTree,
        new_key: u64,
        left_child: Option<u64>,
        right_child: Option<u64>,
    ) {
        let key_destination: usize = match self.keys.binary_search(&new_key) {
            Ok(i) => i,
            Err(i) => i,
        };

        self.keys.insert(key_destination, new_key);
        if self.children.len() == 0 {
            self.children.insert(key_destination, left_child);
        }
        self.children.insert(key_destination + 1, right_child);

        btree.nodes_file.update_node(self);
    }

    fn get_compensation_partners(&mut self, btree: &mut BTree) -> Result<(Node, usize, Node), ()> {
        match self.parent_node_id {
            Some(parent_id) => {
                let parent = btree.nodes_file.get_node(parent_id);

                if parent.keys.len() == 1 {
                    return Err(());
                }

                let position_in_parent: usize =
                    parent.children.binary_search(&Some(self.id)).unwrap();

                if position_in_parent > 0 {
                    let sibling_left = btree.nodes_file.get_node(position_in_parent as u64 - 1);
                    if sibling_left.keys.len() == 2 * btree.order {
                        if position_in_parent < 2 * btree.order {
                            let sibling_right =
                                btree.nodes_file.get_node(position_in_parent as u64 + 1);
                            if sibling_right.keys.len() == 2 * btree.order {
                                return Err(());
                            }
                            return Ok((parent, position_in_parent, sibling_left));
                        }
                        return Err(());
                    }
                    return Ok((parent, position_in_parent - 1, sibling_left));
                } else {
                    let sibling = btree.nodes_file.get_node(position_in_parent as u64 + 1);
                    if sibling.keys.len() == 2 * btree.order {
                        return Err(());
                    }
                    return Ok((parent, 0, sibling));
                }
            }
            None => Err(()),
        }
    }

    fn compensate_insertion(&mut self, btree: &mut BTree) -> Result<(), ()> {
        todo!();
        /*let (mut parent, key_position_in_parent, mut sibling) =
                    self.get_compensation_partners(btree)?;

                // create pools
                let mut key_pool = self.keys.clone();
                key_pool.push(parent.keys[key_position_in_parent]);
                key_pool.append(&mut sibling.keys);
                let mut children_pool = self.children.clone();
                children_pool.append(&mut sibling.children);

                Self::insert_into_sorted(&mut key_pool, new_key);

                // distribute from pools
                let split_point = key_pool.len() / 2;

                sibling.keys = key_pool.split_off(split_point + 1);
                sibling.children = children_pool.split_off(split_point);
                parent.keys[key_position_in_parent] = key_pool.split_off(split_point)[0];
                self.keys = key_pool;
                self.children = children_pool;

                // update in files
                btree.nodes_file.update_node(self);
                btree.nodes_file.update_node(&sibling);
                btree.nodes_file.update_node(&parent);

                Ok(())
        */
    }

    fn insert_into_sorted(dest: &mut Vec<u64>, new_key: u64) {
        let insert_idx = match dest.binary_search(&new_key) {
            Ok(i) => i,
            Err(i) => i,
        };
        dest.insert(insert_idx, new_key);
    }

    fn split(&mut self, btree: &mut BTree) {
        let mut new_node = Node::new(self.is_leaf, btree.nodes_file.next_id, btree.order);
        btree.nodes_file.create_node(&new_node);

        let mut parent = match self.parent_node_id {
            Some(parent_id) => btree.nodes_file.get_node(parent_id),
            None => btree.create_new_root(false),
        };

        new_node.keys = self.keys.split_off(btree.order + 1);
        new_node.children = self.children.split_off(btree.order + 1);

        parent.insert(btree, self.keys.split_off(btree.order)[0], Some(self.id), Some(new_node.id)).unwrap();

        self.parent_node_id = Some(parent.id);
        new_node.parent_node_id = Some(parent.id);

        btree.nodes_file.update_node(&new_node);
        btree.nodes_file.update_node(self);
    }

    fn handle_overflow(&mut self, btree: &mut BTree) {
        if self.keys.len() <= 2 * btree.order {
            return;
        }

        /*if let Ok(_) = self.compensate_insertion(btree) {
            return;
        }*/

        self.split(btree);
    }

    fn insert(
        &mut self,
        btree: &mut BTree,
        new_key: u64,
        left_child: Option<u64>,
        right_child: Option<u64>,
    ) -> Result<(), ()> {
        self.basic_insert(btree, new_key, left_child, right_child);
        btree.nodes_file.update_node(self);
        self.handle_overflow(btree);
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct BTree {
    root_id: Option<u64>,
    nodes_file: NodesFile,
    order: usize,
}

impl BTree {
    pub fn new(nodes_file_name: &Path, order: usize) -> Self {
        Self {
            root_id: None,
            nodes_file: NodesFile::new(nodes_file_name),
            order,
        }
    }

    pub fn get_next_id(&self) -> u64 {
        self.nodes_file.next_id
    }

    pub fn get_node(&mut self, id: u64) -> Node {
        self.nodes_file.get_node(id)
    }

    pub fn insert(&mut self, key: u64) -> Result<(), ()> {
        match self.search(key) {
            Ok((address, node_id)) => Err(()),
            Err((address, node_id)) => {
                let mut root = match self.root_id {
                    Some(id) => self.nodes_file.get_node(id),
                    None => self.create_new_root(true),
                };

                root.insert(self, key, None, None)
            }
        }
    }

    pub fn create_new_root(&mut self, is_leaf: bool) -> Node {
        let mut root = Node::new(is_leaf, self.get_next_id(), self.order);
        self.nodes_file.create_node(&root);
        self.root_id = Some(root.id);
        root
    }

    pub fn search(&mut self, key: u64) -> Result<(u64, u64), (u64, u64)> {
        match self.root_id {
            Some(id) => self.get_node(id).search(self, key),
            None => Err((0, 0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    mod search_tests {
        use super::*;

        #[test]
        fn search_empty() {
            let path = Path::new("test_files/search_empty.json");
            let mut btree = BTree::new(&path, 3);

            let result = btree.search(1);

            assert_eq!(result, Err((0, 0)));
        }

        #[test]
        fn search_root_find() {
            let path = Path::new("test_files/search_empty.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();

            let result = btree.search(2);

            assert_eq!(result, Ok((1, 0)));
        }

        #[test]
        fn search_root_not_found() {
            let path = Path::new("test_files/search_empty.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(3).unwrap();
            btree.insert(4).unwrap();

            let result = btree.search(2);

            assert_eq!(result, Err((1, 0)));
        }
    }

    mod insert_tests {
        use super::*;
        #[test]
        fn insert_into_empty() {
            let path = Path::new("test_files/insert_into_empty.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[1],\"children\":[null,null],\"is_leaf\":true,\"id\":0}]".to_string() ;
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_existing() {
            let path = Path::new("test_files/insert_existing.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            assert!(btree.insert(1).is_err());

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[1],\"children\":[null,null],\"is_leaf\":true,\"id\":0}]".to_string() ;
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_existing_root() {
            let path = Path::new("test_files/insert_into_existing_root.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(3).unwrap();
            btree.insert(0).unwrap();
            btree.insert(2).unwrap();

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[0,1,2,3],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0}]".to_string() ;
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_left() {
            let path = Path::new("test_files/insert_into_full_root_left.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(3).unwrap();
            btree.insert(5).unwrap();
            btree.insert(7).unwrap();
            btree.insert(9).unwrap();
            btree.insert(11).unwrap();

            btree.insert(2).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,2,3],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[7,9,11],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[5],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_right() {
            let path = Path::new("test_files/insert_into_full_root_right.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(3).unwrap();
            btree.insert(5).unwrap();
            btree.insert(7).unwrap();
            btree.insert(9).unwrap();
            btree.insert(11).unwrap();

            btree.insert(10).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,3,5],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[9,10,11],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[7],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_middle() {
            let path = Path::new("test_files/insert_into_full_root_middle.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(3).unwrap();
            btree.insert(5).unwrap();
            btree.insert(7).unwrap();
            btree.insert(9).unwrap();
            btree.insert(11).unwrap();

            btree.insert(6).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,3,5],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[7,9,11],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[6],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_leaf() {
            let path = Path::new("test_files/insert_into_full_leaf.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(1).unwrap();
            btree.insert(3).unwrap();
            btree.insert(5).unwrap();
            btree.insert(7).unwrap();

            btree.insert(9).unwrap();
            btree.insert(11).unwrap();
            btree.insert(13).unwrap();
            btree.insert(15).unwrap();

            btree.insert(17).unwrap();
            btree.insert(18).unwrap();
            btree.insert(19).unwrap();
            btree.insert(20).unwrap();

            let correct_btree = "";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }
    }
}
