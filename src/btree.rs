use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{btree, nodes_file::NodesFile};

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

    fn get_compensation_partners_deletion(
        &mut self,
        btree: &mut BTree,
    ) -> Result<(Node, usize, Node), ()> {
        match self.parent_node_id {
            Some(parent_id) => {
                let parent = btree.nodes_file.get_node(parent_id);
                println!("parent found: {:?}", parent);

                let position_in_parent: usize = parent
                    .children
                    .iter()
                    .position(|c| c == &Some(self.id))
                    .unwrap();

                if position_in_parent > 0 {
                    let sibling_left = btree.nodes_file.get_node(position_in_parent as u64 - 1);
                    if sibling_left.keys.len() == btree.order {
                        if position_in_parent < parent.children.len() - 1 {
                            let sibling_right =
                                btree.nodes_file.get_node(position_in_parent as u64 + 1);
                            if sibling_right.keys.len() == btree.order {
                                return Err(());
                            }
                            return Ok((parent, position_in_parent, sibling_left));
                        }
                        return Err(());
                    }
                    return Ok((parent, position_in_parent - 1, sibling_left));
                } else {
                    let sibling = btree.nodes_file.get_node(position_in_parent as u64 + 1);
                    if sibling.keys.len() == btree.order {
                        return Err(());
                    }
                    return Ok((parent, 0, sibling));
                }
            }
            None => Err(()),
        }
    }

    fn get_compensation_partners_insertion(
        &mut self,
        btree: &mut BTree,
    ) -> Result<(Node, usize, Node), ()> {
        match self.parent_node_id {
            Some(parent_id) => {
                let parent = btree.nodes_file.get_node(parent_id);

                let position_in_parent: usize = parent
                    .children
                    .iter()
                    .position(|c| c == &Some(self.id))
                    .unwrap();

                println!("Parent: {:?}", parent);
                println!("Position in parent: {:?}", position_in_parent);

                if position_in_parent > 0 {
                    let sibling_left = btree.nodes_file.get_node(position_in_parent as u64 - 1);
                    if sibling_left.keys.len() == 2 * btree.order {
                        if position_in_parent < parent.children.len() - 1 {
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

    fn compensate(
        &mut self,
        btree: &mut BTree,
        parent: &mut Node,
        key_position_in_parent: usize,
        sibling: &mut Node,
    ) -> Result<(), ()> {
        let mut key_pool;
        let mut children_pool;

        let (smaller_sib, larger_sib) = if self.keys[0] <= sibling.keys[0] {
            (self, sibling)
        } else {
            (sibling, self)
        };

        key_pool = smaller_sib.keys.split_off(0);
        children_pool = smaller_sib.children.split_off(0);

        key_pool.push(parent.keys[key_position_in_parent]);
        key_pool.append(&mut larger_sib.keys.split_off(0));
        children_pool.append(&mut larger_sib.children.split_off(0));

        println!("Key pool: {:?}", key_pool);
        println!("Children pool: {:?}", children_pool);

        let split_point = key_pool.len();

        larger_sib.keys = key_pool.split_off(split_point / 2 + 1);
        larger_sib.children = children_pool.split_off(split_point / 2 + 1);

        parent.keys[key_position_in_parent] = key_pool.split_off(split_point/2)[0];

        smaller_sib.keys = key_pool.split_off(0);
        smaller_sib.children = children_pool.split_off(0);

        btree.nodes_file.update_node(smaller_sib);
        btree.nodes_file.update_node(larger_sib);
        btree.nodes_file.update_node(&parent);

        Ok(())
    }

    fn compensate_insertion(&mut self, btree: &mut BTree) -> Result<(), ()> {
        let (mut parent, key_position_in_parent, mut sibling) =
            self.get_compensation_partners_insertion(btree)?;

        self.compensate(btree, &mut parent, key_position_in_parent, &mut sibling)
    }

    fn compensate_deletion(&mut self, btree: &mut BTree) -> Result<(), ()> {
        println!("Try compensating deletion");
        let (mut parent, key_position_in_parent, mut sibling) =
            self.get_compensation_partners_deletion(btree)?;
        println!("Compensating deletion");

        self.compensate(btree, &mut parent, key_position_in_parent, &mut sibling)
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

        parent
            .insert(
                btree,
                self.keys.split_off(btree.order)[0],
                Some(self.id),
                Some(new_node.id),
            )
            .unwrap();

        self.parent_node_id = Some(parent.id);
        new_node.parent_node_id = Some(parent.id);

        btree.nodes_file.update_node(&new_node);
        btree.nodes_file.update_node(self);
    }

    fn handle_overflow(&mut self, btree: &mut BTree) {
        if self.keys.len() > 2 * btree.order {
            if let Ok(_) = self.compensate_insertion(btree) {
                return;
            }

            self.split(btree);
        } else if self.keys.len() < btree.order && self.id != btree.root_id.unwrap() {
            if let Ok(_) = self.compensate_deletion(btree) {
                return;
            }

            self.merge(btree);
        }
    }

    fn merge(&mut self, btree: &mut BTree) {
        todo!();
    }

    fn insert(
        &mut self,
        btree: &mut BTree,
        new_key: u64,
        left_child: Option<u64>,
        right_child: Option<u64>,
    ) -> Result<(), ()> {
        println!("Insert {:?} began", new_key);

        self.basic_insert(btree, new_key, left_child, right_child);
        btree.nodes_file.update_node(self);
        self.handle_overflow(btree);

        println!("Insert {:?} finished", new_key);
        return Ok(());
    }

    fn get_smallest_in_subtree(&self, btree: &mut BTree) -> (u64, u64) {
        match self.children[0] {
            Some(child_id) => btree
                .nodes_file
                .get_node(child_id)
                .get_smallest_in_subtree(btree),
            None => (self.keys[0], self.id),
        }
    }

    fn get_largest_in_subtree(&self, btree: &mut BTree) -> (u64, u64) {
        match self.children.last().unwrap() {
            Some(child_id) => btree
                .nodes_file
                .get_node(child_id.clone())
                .get_largest_in_subtree(btree),
            None => (self.keys.last().unwrap().clone(), self.id),
        }
    }

    fn basic_delete(&mut self, btree: &mut BTree, deleted_key: u64) -> u64 {
        let key_position: usize = self.keys.binary_search(&deleted_key).unwrap();
        match self.is_leaf {
            true => {
                self.keys.remove(key_position);
                self.children.remove(key_position);

                if self.keys.len() == 0 {
                    self.children.remove(0);
                }

                btree.nodes_file.update_node(self);
                self.id
            }
            false => {
                let sibling = btree.get_node(self.children[key_position + 1].unwrap());
                let (key, node_id) = sibling.get_smallest_in_subtree(btree);

                let mut borrowing_leaf = btree.get_node(node_id);

                self.keys[key_position] = key;

                borrowing_leaf.keys.remove(0);
                borrowing_leaf.children.remove(0);
                if borrowing_leaf.keys.len() == 0 {
                    borrowing_leaf.children.remove(0);
                }

                btree.nodes_file.update_node(self);
                btree.nodes_file.update_node(&borrowing_leaf);

                node_id
            }
        }
    }

    fn delete(&mut self, btree: &mut BTree, deleted_key: u64) -> Result<(), ()> {
        println!("Delete {:?} began", deleted_key);

        let deletion_node_id = self.basic_delete(btree, deleted_key);
        let mut deletion_node = btree.get_node(deletion_node_id);
        btree.nodes_file.update_node(&deletion_node);
        deletion_node.handle_overflow(btree);

        println!("Delete {:?} finished", deleted_key);
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
                if let None = self.root_id {
                    self.create_new_root(true);
                }
                let mut node = self.nodes_file.get_node(node_id);
                node.insert(self, key, None, None)
            }
        }
    }

    pub fn delete(&mut self, key: u64) -> Result<(), ()> {
        let (key_pos, node_id) = self.search(key).map_err(|e| ())?;

        self.get_node(node_id).delete(self, key);
        Ok(())
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
            let path = Path::new("test_files/search_root_find.json");
            let mut btree = BTree::new(&path, 3);

            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();

            let result = btree.search(2);

            assert_eq!(result, Ok((1, 0)));
        }

        #[test]
        fn search_root_not_found() {
            let path = Path::new("test_files/search_root_not_found.json");
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
        fn insert_into_full_leaf_split() {
            let path = Path::new("test_files/insert_into_full_leaf_split.json");
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

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,3,5,7],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[11,13],\"children\":[null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[9,15],\"children\":[0,1,3],\"is_leaf\":false,\"id\":2},{\"parent_node_id\":2,\"keys\":[17,18],\"children\":[null,null,null],\"is_leaf\":true,\"id\":3}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_leaf_compensation() {
            let path = Path::new("test_files/insert_into_full_leaf_compensation.json");
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

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,3,5,7],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[11,13],\"children\":[null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[9,15],\"children\":[0,1,3],\"is_leaf\":false,\"id\":2},{\"parent_node_id\":2,\"keys\":[17,18,19,20],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":3}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }
    }

    mod delete_tests {
        use super::*;

        #[test]
        fn delete_from_empty() {
            let path = Path::new("test_files/delete_from_empty.json");
            let mut btree = BTree::new(&path, 2);

            assert!(btree.delete(0).is_err());

            let correct_btree = "";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_non_existent() {
            let path = Path::new("test_files/delete_non_existent.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            assert!(btree.delete(6).is_err());

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,2],\"children\":[null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[4,5],\"children\":[null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[3],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_from_root() {
            let path = Path::new("test_files/delete_from_root.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();
            btree.insert(4).unwrap();

            btree.delete(3).unwrap();

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[1,2,4],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":0}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_from_leaf() {
            let path = Path::new("test_files/delete_from_leaf.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            btree.insert(6).unwrap();

            btree.delete(2).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[1,3],\"children\":[null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[5,6],\"children\":[null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[4],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_from_middle() {
            let path = Path::new("test_files/delete_from_middle.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(0).unwrap();
            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            btree.insert(6).unwrap();
            btree.insert(7).unwrap();
            btree.insert(8).unwrap();

            btree.delete(5).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[0,1,2,3],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[6,7,8],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[4],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }
    }

    mod update_tests {
        use super::*;
    }
}
