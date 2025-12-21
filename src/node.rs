use std::{array::TryFromSliceError, error::Error, fmt::format, u64};

use serde::{Deserialize, Serialize};

use crate::{
    btree::{self, BTree},
    record::{RECORD_SIZE, Record},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Node {
    pub id: u64,
    pub parent_node_id: Option<u64>,
    pub is_leaf: bool,
    pub is_deleted: bool,
    keys: Vec<Record>,
    pub children: Vec<Option<u64>>,
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
                    let child = btree.get_node(child_id);
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

        btree.update_node(self);
    }

    fn get_compensation_partners_deletion(
        &mut self,
        btree: &mut BTree,
    ) -> Result<(Node, usize, Node), ()> {
        match self.parent_node_id {
            Some(parent_id) => {
                let parent = btree.get_node(parent_id);

                let position_in_parent: usize = parent
                    .children
                    .iter()
                    .position(|c| c == &Some(self.id))
                    .unwrap();

                if position_in_parent > 0 {
                    let sibling_left =
                        btree.get_node(parent.children[position_in_parent - 1].unwrap());
                    if sibling_left.keys.len() <= btree.order {
                        if position_in_parent < parent.children.len() - 1 {
                            let sibling_right =
                                btree.get_node(parent.children[position_in_parent + 1].unwrap());
                            if sibling_right.keys.len() <= btree.order {
                                return Err(());
                            }
                            return Ok((parent, position_in_parent, sibling_right));
                        }
                        return Err(());
                    }
                    return Ok((parent, position_in_parent - 1, sibling_left));
                } else {
                    let sibling = btree.get_node(parent.children[position_in_parent + 1].unwrap());
                    if sibling.keys.len() <= btree.order {
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
                let parent = btree.get_node(parent_id);

                let position_in_parent: usize = parent
                    .children
                    .iter()
                    .position(|c| c == &Some(self.id))
                    .unwrap();

                if position_in_parent > 0 {
                    let sibling_left =
                        btree.get_node(parent.children[position_in_parent - 1].unwrap());
                    if sibling_left.keys.len() >= 2 * btree.order {
                        if position_in_parent < parent.children.len() - 1 {
                            let sibling_right =
                                btree.get_node(parent.children[position_in_parent + 1].unwrap());
                            if sibling_right.keys.len() >= 2 * btree.order {
                                return Err(());
                            }
                            return Ok((parent, position_in_parent, sibling_right));
                        }
                        return Err(());
                    }
                    return Ok((parent, position_in_parent - 1, sibling_left));
                } else {
                    let sibling = btree.get_node(parent.children[position_in_parent + 1].unwrap());
                    if sibling.keys.len() >= 2 * btree.order {
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

        let split_point = key_pool.len();

        larger_sib.keys = key_pool.split_off(split_point / 2 + 1);
        larger_sib.children = children_pool.split_off(split_point / 2 + 1);
        for child_id in larger_sib.children.iter() {
            if let Some(id) = child_id {
                let mut node = btree.get_node(id.clone());
                node.parent_node_id = Some(larger_sib.id);
                btree.update_node(&node);
            }
        }

        parent.keys[key_position_in_parent] = key_pool.split_off(split_point / 2)[0];

        smaller_sib.keys = key_pool.split_off(0);
        smaller_sib.children = children_pool.split_off(0);
        for child_id in smaller_sib.children.iter() {
            if let Some(id) = child_id {
                let mut node = btree.get_node(id.clone());
                node.parent_node_id = Some(smaller_sib.id);
                btree.update_node(&node);
            }
        }

        btree.update_node(smaller_sib);
        btree.update_node(larger_sib);
        btree.update_node(&parent);

        Ok(())
    }

    fn compensate_insertion(&mut self, btree: &mut BTree) -> Result<(), ()> {
        let (mut parent, key_position_in_parent, mut sibling) =
            self.get_compensation_partners_insertion(btree)?;

        self.compensate(btree, &mut parent, key_position_in_parent, &mut sibling)
    }

    fn compensate_deletion(&mut self, btree: &mut BTree) -> Result<(), ()> {
        let (mut parent, key_position_in_parent, mut sibling) =
            self.get_compensation_partners_deletion(btree)?;

        self.compensate(btree, &mut parent, key_position_in_parent, &mut sibling)
    }

    fn split(&mut self, btree: &mut BTree) {
        let mut new_node = Node::new(self.is_leaf, btree.nodes_file.next_id, btree.order);
        btree.create_node(&new_node);

        let mut parent = match self.parent_node_id {
            Some(parent_id) => btree.get_node(parent_id),
            None => btree.create_new_root(false),
        };

        new_node.keys = self.keys.split_off(btree.order + 1);
        new_node.children = self.children.split_off(btree.order + 1);

        for child_id in new_node.children.iter() {
            if let Some(id) = child_id {
                let mut node = btree.get_node(id.clone());
                node.parent_node_id = Some(new_node.id);
                btree.update_node(&node);
            }
        }
        let parent_record = self.keys.split_off(btree.order)[0];

        self.parent_node_id = Some(parent.id);
        new_node.parent_node_id = Some(parent.id);

        btree.update_node(&new_node);
        btree.update_node(self);
        btree.update_node(&parent);

        parent
            .insert(btree, parent_record, Some(self.id), Some(new_node.id))
            .unwrap();
    }

    pub fn keys_string_pretty(&self) -> String {
        let mut str = String::new();
        for key in self.keys.iter() {
            str += format!("[{}: {}]", key.key, key.data_id).as_str();
        }

        str
    }

    pub fn print(&self, btree: &mut BTree, ident: u8) {
        println!(
            "{}{}| id: {} keys: {:?}",
            vec!['-'; 2 * ident as usize].iter().collect::<String>(),
            self.parent_node_id
                .map_or(String::from("X"), |id| { id.to_string() }),
            self.id,
            self.keys_string_pretty(),
        );
        if !self.is_leaf {
            for child_id in self.children.iter() {
                btree.get_node(child_id.unwrap()).print(btree, ident + 1);
            }
        }
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
        } else if self.keys.len() == 0 {
            btree.root_id = self.children[0];
            if let Some(child_id) = self.children[0] {
                let mut node = btree.get_node(child_id);
                node.parent_node_id = None;
                btree.update_node(&node);
            }
            self.is_deleted = true;
            btree.update_node(self);
        }
    }

    fn get_parent(&mut self, btree: &mut BTree) -> (Node, usize) {
        let parent = btree.get_node(self.parent_node_id.unwrap());

        let position_in_parent: usize = parent
            .children
            .iter()
            .position(|c| c == &Some(self.id))
            .unwrap();

        (parent, position_in_parent)
    }

    pub fn print_in_order(&self, btree: &mut BTree) {
        for i in 0..self.keys.len() {
            if !self.is_leaf {
                btree
                    .get_node(self.children[i].unwrap())
                    .print_in_order(btree);
            }
            let data = btree.data_file.get_data(&self.keys[i]).unwrap();
            println!("{}: {}", self.keys[i].key, data);
        }
        if !self.is_leaf {
            btree
                .get_node(self.children.last().unwrap().unwrap())
                .print_in_order(btree);
        }
    }

    fn merge(&mut self, btree: &mut BTree) {
        let (mut parent, position_in_parent) = self.get_parent(btree);

        if position_in_parent < parent.keys.len() {
            let mut sibling = btree.get_node(parent.children[position_in_parent + 1].unwrap());

            self.keys.push(parent.keys.remove(position_in_parent));
            parent.children.remove(position_in_parent + 1);

            self.keys.append(&mut sibling.keys);
            for child_id in sibling.children.iter_mut() {
                if let Some(id) = child_id {
                    let mut node = btree.get_node(id.clone());
                    node.parent_node_id = Some(self.id);
                    btree.update_node(&node);
                }
            }
            self.children.append(&mut sibling.children);

            sibling.is_deleted = true;
            btree.update_node(&sibling);
        } else {
            let mut sibling = btree.get_node(parent.children[position_in_parent - 1].unwrap());

            sibling
                .keys
                .push(parent.keys.remove(position_in_parent - 1));
            parent.children.remove(position_in_parent);

            sibling.keys.append(&mut self.keys);
            for child_id in self.children.iter_mut() {
                if let Some(id) = child_id {
                    let mut node = btree.get_node(id.clone());
                    node.parent_node_id = Some(sibling.id);
                    btree.update_node(&node);
                }
            }
            sibling.children.append(&mut self.children);

            self.is_deleted = true;
            btree.update_node(&sibling);
        };

        btree.update_node(self);
        btree.update_node(&parent);

        parent.handle_overflow(btree);
    }

    pub fn insert(
        &mut self,
        btree: &mut BTree,
        new_record: Record,
        left_child: Option<u64>,
        right_child: Option<u64>,
    ) -> Result<(), ()> {
        self.basic_insert(btree, &new_record, left_child, right_child);
        self.handle_overflow(btree);
        return Ok(());
    }

    pub fn get_smallest_in_subtree(&self, btree: &mut BTree) -> (Record, u64) {
        match self.children[0] {
            Some(child_id) => btree.get_node(child_id).get_smallest_in_subtree(btree),
            None => (self.keys[0].clone(), self.id),
        }
    }

    pub fn get_largest_in_subtree(&self, btree: &mut BTree) -> (Record, u64) {
        match self.children.last().unwrap() {
            Some(child_id) => btree
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

                btree.update_node(self);
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

                btree.update_node(self);
                btree.update_node(&borrowing_leaf);

                node_id
            }
        }
    }

    pub fn delete(&mut self, btree: &mut BTree, deleted_key: u64) -> Result<(), ()> {
        let deletion_node_id = self.basic_delete(btree, deleted_key);
        let mut deletion_node = btree.get_node(deletion_node_id);
        btree.update_node(&deletion_node);
        deletion_node.handle_overflow(btree);

        return Ok(());
    }

    pub fn byte_size(order: usize) -> usize {
        1 + 8 + 8 + 8 + (2 * order + 1) * RECORD_SIZE + (2 * order + 2) * 8
    }

    // 1 - flags: is_leaf, is_deleted, has_parent
    // 8 - id
    // 8 - parent_node_id
    // 8 - key_count
    //
    // MAX_KEY_COUNT * KEY_SIZE - keys
    // (MAX_KEY_COUNT + 1) * KEY_SIZE - children
    pub fn to_bytes(&self, order: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(Self::byte_size(order));

        let max_key_count = order * 2 + 1;
        let max_child_count = max_key_count + 1;

        let flags: u8 = [self.is_leaf, self.is_deleted, self.parent_node_id.is_some()]
            .iter()
            .enumerate()
            .map(|f| if *f.1 { 1 << f.0 } else { 0 })
            .reduce(|acc, e| acc | e)
            .unwrap();

        bytes.push(flags);
        bytes.append(&mut self.id.to_le_bytes().to_vec());
        bytes.append(&mut self.parent_node_id.unwrap_or(0).to_le_bytes().to_vec());
        bytes.append(&mut (self.keys.len() as u64).to_le_bytes().to_vec());

        let mut record_bytes: Vec<u8> = self.keys.iter().flat_map(|r| Vec::from(r)).collect();
        record_bytes.resize(max_key_count * RECORD_SIZE, 0);
        bytes.append(&mut record_bytes);

        let mut child_bytes: Vec<u8> = self
            .children
            .iter()
            .flat_map(|c| c.unwrap_or(0).to_le_bytes())
            .collect();
        child_bytes.resize(max_child_count * 8, 0);
        bytes.append(&mut child_bytes);

        bytes
    }

    pub fn from_bytes(bytes: &[u8], order: usize) -> Self {
        let flags: u8 = bytes[0];
        let is_leaf = (flags & 1 << 0) > 0;
        let is_deleted = (flags & 1 << 1) > 0;
        let has_parent = (flags & 1 << 2) > 0;

        let id = u64::from_le_bytes(bytes[1..9].try_into().unwrap());

        let parent_node_id = if has_parent {
            Some(u64::from_le_bytes(bytes[9..17].try_into().unwrap()))
        } else {
            None
        };

        let key_count = u64::from_le_bytes(bytes[17..25].try_into().unwrap());

        let max_key_count = order * 2 + 1;

        let record_bytes = bytes[25..(25 + key_count as usize * RECORD_SIZE)].chunks(RECORD_SIZE);
        let children_bytes = bytes[(25 + max_key_count * RECORD_SIZE)
            ..(25 + max_key_count * RECORD_SIZE) + 8 * (key_count as usize + 1)]
            .chunks(8);

        let keys = record_bytes.map(|b| Record::try_from(b).unwrap()).collect();
        let children = if !is_leaf {
            children_bytes
                .map(|b| Some(u64::from_le_bytes(b.try_into().unwrap())))
                .collect()
        } else {
            children_bytes.map(|_| None).collect()
        };

        Self {
            id,
            parent_node_id,
            is_leaf,
            is_deleted,
            keys,
            children,
        }
    }
}
