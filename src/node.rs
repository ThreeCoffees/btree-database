use serde::{Deserialize, Serialize};

use crate::{
    btree::{self, BTree},
    record::Record,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub parent_node_id: Option<u64>,
    keys: Vec<Record>,
    pub children: Vec<Option<u64>>,
    pub is_leaf: bool,
    pub is_deleted: bool,
    pub id: u64,
}

impl Node {
    pub fn new(is_leaf: bool, id: u64, order: usize) -> Self {
        Self {
            parent_node_id: None,
            keys: Vec::with_capacity(order * 2 + 1),
            children: Vec::with_capacity(order * 2 + 2),
            is_leaf,
            is_deleted: false,
            id,
        }
    }

    pub fn record_count(&self) -> usize {
        self.keys.len()
    }

    pub fn search(&self, btree: &mut BTree, key: u64) -> Result<(Record, u64), (u64, u64)> {
        match self.keys.binary_search_by(|r| r.key.cmp(&key)) {
            Ok(i) => return Ok((self.keys[i], self.id)),
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
        new_record: &Record,
        left_child: Option<u64>,
        right_child: Option<u64>,
    ) {
        let key_destination: usize = match self.keys.binary_search(&new_record) {
            Ok(i) => i,
            Err(i) => i,
        };

        self.keys.insert(key_destination, new_record.clone());
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
                //println!("parent found: {:?}", parent);

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

                //println!("Parent: {:?}", parent);
                //println!("Position in parent: {:?}", position_in_parent);

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

        //println!("Key pool: {:?}", key_pool);
        //println!("Children pool: {:?}", children_pool);

        let split_point = key_pool.len();

        larger_sib.keys = key_pool.split_off(split_point / 2 + 1);
        larger_sib.children = children_pool.split_off(split_point / 2 + 1);

        parent.keys[key_position_in_parent] = key_pool.split_off(split_point / 2)[0];

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
        //println!("Try compensating deletion");
        let (mut parent, key_position_in_parent, mut sibling) =
            self.get_compensation_partners_deletion(btree)?;
        //println!("Compensating deletion");

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

        let parent_record = self.keys.split_off(btree.order)[0];

        parent
            .insert(btree, parent_record, Some(self.id), Some(new_node.id))
            .unwrap();

        self.parent_node_id = Some(parent.id);
        new_node.parent_node_id = Some(parent.id);

        btree.nodes_file.update_node(&new_node);
        btree.nodes_file.update_node(self);
    }

    fn handle_overflow(&mut self, btree: &mut BTree) {
        println!("overflow check");
        if self.keys.len() > 2 * btree.order {
            if let Ok(_) = self.compensate_insertion(btree) {
                return;
            }

            self.split(btree);
        } else if self.keys.len() < btree.order && self.id != btree.root_id.unwrap() {
            if let Ok(_) = self.compensate_deletion(btree) {
                return;
            }

            println!("handling overflow");
            self.merge(btree);
        } else if self.keys.len() == 0 {
            println!("handling full delete");
            btree.root_id = self.children[0];
            if let Some(child_id) = self.children[0] {
                let mut node = btree.get_node(child_id);
                node.parent_node_id = None;
                btree.nodes_file.update_node(&node);
            }
            self.is_deleted = true;
            btree.nodes_file.update_node(self);
        }
    }

    fn get_parent(&mut self, btree: &mut BTree) -> (Node, usize) {
        let parent = btree.nodes_file.get_node(self.parent_node_id.unwrap());

        let position_in_parent: usize = parent
            .children
            .iter()
            .position(|c| c == &Some(self.id))
            .unwrap();

        (parent, position_in_parent)
    }

    fn merge(&mut self, btree: &mut BTree) {
        let (mut parent, position_in_parent) = self.get_parent(btree);

        if position_in_parent < parent.keys.len() {
            let mut sibling = btree.nodes_file.get_node(position_in_parent as u64 + 1);

            self.keys.push(parent.keys.remove(position_in_parent));
            parent.children.remove(position_in_parent + 1);

            self.keys.append(&mut sibling.keys);
            for child_id in sibling.children.iter_mut() {
                if let Some(id) = child_id {
                    let mut node = btree.get_node(id.clone());
                    node.parent_node_id = Some(self.id);
                    btree.nodes_file.update_node(&node);
                }
            }
            self.children.append(&mut sibling.children);

            sibling.is_deleted = true;
            btree.nodes_file.update_node(&sibling);
        } else {
            let mut sibling = btree.nodes_file.get_node(position_in_parent as u64 - 1);

            sibling
                .keys
                .push(parent.keys.remove(position_in_parent - 1));
            parent.children.remove(position_in_parent);

            sibling.keys.append(&mut self.keys);
            for child_id in self.children.iter_mut() {
                if let Some(id) = child_id {
                    let mut node = btree.get_node(id.clone());
                    node.parent_node_id = Some(sibling.id);
                    btree.nodes_file.update_node(&node);
                }
            }
            sibling.children.append(&mut self.children);

            self.is_deleted = true;
            btree.nodes_file.update_node(&sibling);
        };

        btree.nodes_file.update_node(self);
        btree.nodes_file.update_node(&parent);

        parent.handle_overflow(btree);
    }

    pub fn insert(
        &mut self,
        btree: &mut BTree,
        new_record: Record,
        left_child: Option<u64>,
        right_child: Option<u64>,
    ) -> Result<(), ()> {
        //println!("Insert {:?} began", new_record.key);

        self.basic_insert(btree, &new_record, left_child, right_child);
        btree.nodes_file.update_node(self);
        self.handle_overflow(btree);

        //println!("Insert {:?} finished", new_record.key);
        return Ok(());
    }

    pub fn get_smallest_in_subtree(&self, btree: &mut BTree) -> (Record, u64) {
        match self.children[0] {
            Some(child_id) => btree
                .nodes_file
                .get_node(child_id)
                .get_smallest_in_subtree(btree),
            None => (self.keys[0].clone(), self.id),
        }
    }

    pub fn get_largest_in_subtree(&self, btree: &mut BTree) -> (Record, u64) {
        match self.children.last().unwrap() {
            Some(child_id) => btree
                .nodes_file
                .get_node(child_id.clone())
                .get_largest_in_subtree(btree),
            None => (self.keys.last().unwrap().clone(), self.id),
        }
    }

    fn basic_delete(&mut self, btree: &mut BTree, deleted_key: u64) -> u64 {
        let key_position: usize = self
            .keys
            .binary_search_by(|r| r.key.cmp(&deleted_key))
            .unwrap();
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

    pub fn delete(&mut self, btree: &mut BTree, deleted_key: u64) -> Result<(), ()> {
        //println!("Delete {:?} began", deleted_key);

        let deletion_node_id = self.basic_delete(btree, deleted_key);
        let mut deletion_node = btree.get_node(deletion_node_id);
        btree.nodes_file.update_node(&deletion_node);
        deletion_node.handle_overflow(btree);

        //println!("Delete {:?} finished", deleted_key);
        return Ok(());
    }
}
