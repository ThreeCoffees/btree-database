use std::{cmp::min, path::Path};

use serde::{Deserialize, Serialize};

use crate::{btree, nodes_file::NodesFile};

const D: usize = 3;

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
    fn new(is_leaf: bool, id: u64) -> Self {
        Self {
            parent_node_id: None,
            keys: Vec::with_capacity(D * 2 + 1),
            children: Vec::with_capacity(D * 2 + 2),
            is_leaf,
            id,
        }
    }

    /*
    fn split_insert(&mut self, btree: &mut BTree, new_key: u64) {
        let mut new_node = Node::new(self.is_leaf, btree.get_next_id());
        btree.nodes_file.create_node(&new_node);

        self.keys.len() = D as u64;
        new_node.keys.len() = D as u64;

        let key_destination: usize = match self.keys.binary_search(&new_key) {
            Ok(i) => i,
            Err(i) => i,
        };

        let parent_key: u64 = if key_destination < D {
            //split with middle_left pushed to the parent
            new_node.keys = self.keys.split_off(D);
            let key = self.keys.split_off(D - 1)[0];
            new_node.children = self.children.split_off(D);

            self.non_split_insert(btree, new_key);

            key
        } else if key_destination > D {
            //split with middle_right pushed to the parent
            new_node.keys = self.keys.split_off(D + 1);
            new_node.children = self.children.split_off(D + 1);

            new_node.non_split_insert(btree, new_key);

            self.keys.split_off(D)[0]
        } else {
            //split with new_key pushed to the parent
            new_node.keys = self.keys.split_off(D);
            new_node.children = self.children.split_off(D);

            new_key
        };

        btree.nodes_file.update_node(&new_node);
        btree.nodes_file.update_node(&self);

        // push to the parent
        let mut parent_node: Node = match self.parent_node_id {
            Some(id) => btree.get_node(id),
            None => btree.create_new_root(false),
        };

        parent_node.insert(btree, parent_key);

        todo!()
    }*/

    /*fn split_child(&mut self, child_idx: usize) {
        let child: &mut Node = self.children[child_idx].as_mut().unwrap();
        let mut new_child = Node::new(child.is_leaf);

        child.keys.len() = D as u64;
        new_child.keys.len() = D as u64;

        new_child.keys = child.keys.split_off(D + 1);
        let split_key = child.keys.split_off(D as usize)[0];

        new_child.children = child.children.split_off(D + 1);

        self.keys.insert(child_idx, split_key);
        self.children.insert(child_idx + 1, Some(new_child));

        self.keys.len() += 1;
    }*/

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
        self.children.insert(key_destination, left_child);
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
                    if sibling_left.keys.len() == 2 * D {
                        if position_in_parent < 2 * D {
                            let sibling_right =
                                btree.nodes_file.get_node(position_in_parent as u64 + 1);
                            if sibling_right.keys.len() == 2 * D {
                                return Err(());
                            }
                            return Ok((parent, position_in_parent, sibling_left));
                        }
                        return Err(());
                    }
                    return Ok((parent, position_in_parent - 1, sibling_left));
                } else {
                    let sibling = btree.nodes_file.get_node(position_in_parent as u64 + 1);
                    if sibling.keys.len() == 2 * D {
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
        let mut new_node = Node::new(self.is_leaf, btree.nodes_file.next_id);
        btree.nodes_file.create_node(&new_node);

        let mut parent = match self.parent_node_id {
            Some(parent_id) => btree.nodes_file.get_node(parent_id),
            None => todo!(),
        };

        new_node.keys = self.keys.split_off(D + 1);
        new_node.children = self.children.split_off(D + 1);

        /*let position_in_parent = parent
            .children
            .iter()
            .position(|&c| {
                if let Some(c) = c {
                    return c == self.id;
                }
                return false;
            })
            .unwrap();
*/
        parent.basic_insert(btree, self.keys.split_off(D)[0], Some(self.id), Some(new_node.id));

        /*
        let mut key_pool = self.keys.split_off(0);
        Self::insert_into_sorted(&mut key_pool, new_key);
        let mut children_pool = self.children.split_off(0);

        let split_point = key_pool.len() / 2;

        new_node.keys = key_pool.split_off(split_point + 1);
        new_node.children = children_pool.split_off(split_point);
        let parent_key = key_pool.split_off(split_point)[0];
        self.keys = key_pool;
        self.children = children_pool;

        match self.parent_node_id {
            Some(parent_id) => {
                btree
                    .nodes_file
                    .get_node(parent_id)
                    .insert(btree, parent_key)
                    .unwrap();
            }
            None => {
                btree
                    .create_new_root(false)
                    .insert(btree, parent_key)
                    .unwrap();
            }
        }*/
    }

    fn handle_overflow(&mut self, btree: &mut BTree) {
        if self.keys.len() <= 2 * D {
            return;
        }

        if let Ok(_) = self.compensate_insertion(btree) {
            return;
        }

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
        self.handle_overflow(btree);
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct BTree {
    root_id: Option<u64>,
    nodes_file: NodesFile,
}

impl BTree {
    pub fn new(nodes_file_name: &Path) -> Self {
        Self {
            root_id: None,
            nodes_file: NodesFile::new(nodes_file_name),
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
        let mut root = Node::new(is_leaf, self.get_next_id());
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
    //let root = self.nodes[self.root_id as usize].borrow_mut();
    /*
    match self.root.as_mut() {
        Some(root) => {
            if root.keys.len() == 2 * D as u64 {
                let mut new_root = Node::new(false);
                mem::swap(root, &mut new_root);
                root.children.push(Some(new_root));
                root.split_child(0);
            }
            root.insert_non_full(key);
        }
        None => {
            self.root = Some(Node::new(true));
            self.root.as_mut().unwrap().insert_non_full(key);
        }
    }*/
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
            let mut btree = BTree::new(&path);

            let result = btree.search(1);

            assert_eq!(result, Err((0, 0)));
        }

        #[test]
        fn search_root_find() {
            let path = Path::new("test_files/search_empty.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);
            btree.insert(2);
            btree.insert(3);

            let result = btree.search(2);

            assert_eq!(result, Ok((1, 0)));
        }

        #[test]
        fn search_root_not_found() {
            let path = Path::new("test_files/search_empty.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);
            btree.insert(3);
            btree.insert(4);

            let result = btree.search(2);

            assert_eq!(result, Err((1, 0)));
        }
    }

    mod insert_tests {
        use super::*;
        #[test]
        fn insert_into_empty() {
            let path = Path::new("test_files/insert_into_empty.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);

            let correct_btree = "[{\"keys.len()\":1,\"parent_node_id\":null,\"keys\":[1],\"children\":[null,null],\"is_leaf\":true,\"id\":0}]".to_string() ;
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_existing_root() {
            let path = Path::new("test_files/insert_into_existing_root.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);
            btree.insert(3);
            btree.insert(0);
            btree.insert(2);

            let correct_btree = "[{\"keys.len()\":4,\"parent_node_id\":null,\"keys\":[0,1,2,3],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0}]".to_string() ;
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_left() {
            let path = Path::new("test_files/insert_into_full_root_left.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);
            btree.insert(3);
            btree.insert(5);
            btree.insert(7);
            btree.insert(9);
            btree.insert(11);

            btree.insert(2);

            let correct_btree = "";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_right() {
            let path = Path::new("test_files/insert_into_full_root_right.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);
            btree.insert(3);
            btree.insert(5);
            btree.insert(7);
            btree.insert(9);
            btree.insert(11);

            btree.insert(10);

            let correct_btree = "";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_middle() {
            let path = Path::new("test_files/insert_into_full_root_middle.json");
            let mut btree = BTree::new(&path);

            btree.insert(1);
            btree.insert(3);
            btree.insert(5);
            btree.insert(7);
            btree.insert(9);
            btree.insert(11);

            btree.insert(6);

            let correct_btree = "";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_leaf() {}
    }

    #[test]
    fn split_child() {
        /*
                //setup
                let node1 = Node {
                    keys.len(): 7,
                    keys: vec![0, 1, 2, 3, 4, 5, 6],
                    children: vec![None; 8],
                    is_leaf: true,
                };
                let node2 = Node {
                    keys.len(): 7,
                    keys: vec![8, 9, 10, 11, 12, 13, 14],
                    children: vec![None; 8],
                    is_leaf: true,
                };

                let mut parent_node = Node::new(false);
                parent_node.keys.len() = 1;
                parent_node.keys.push(7);
                parent_node.children.push(Some(node1));
                parent_node.children.push(Some(node2));

                // action

                parent_node.split_child(0);
                parent_node.split_child(2);

                // assert

                let split1 = Node {
                    keys.len(): 3,
                    keys: vec![0, 1, 2],
                    children: vec![None, None, None, None],
                    is_leaf: true,
                };
                let split2 = Node {
                    keys.len(): 3,
                    keys: vec![4, 5, 6],
                    children: vec![None, None, None, None],
                    is_leaf: true,
                };
                let split3 = Node {
                    keys.len(): 3,
                    keys: vec![8, 9, 10],
                    children: vec![None, None, None, None],
                    is_leaf: true,
                };
                let split4 = Node {
                    keys.len(): 3,
                    keys: vec![12, 13, 14],
                    children: vec![None, None, None, None],
                    is_leaf: true,
                };

                let correct_parent_node = Node {
                    keys.len(): 3,
                    keys: vec![3, 7, 11],
                    children: vec![Some(split1), Some(split2), Some(split3), Some(split4)],
                    is_leaf: false,
                };

                assert_eq!(parent_node, correct_parent_node);
        */
    }
}
