#[cfg(test)]
mod tests {
    use std::fs;

    mod search_tests {
        use std::path::Path;

        use crate::{btree::BTree, data::Data};

        #[test]
        fn search_empty() {
            let path = Path::new("test_files/search_empty.json");
            let data_path = Path::new("test_files/search_empty");
            let mut btree = BTree::new(&path, &data_path, 3);

            let result = btree.search(1);

            assert_eq!(result, Err((0, 0)));
        }

        #[test]
        fn search_root_find() {
            let path = Path::new("test_files/search_root_find.json");
            let data_path = Path::new("test_files/search_root_find");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree.insert(1, &Data::try_from("01".as_bytes()).unwrap()).unwrap();
            btree.insert(2, &Data::try_from("02".as_bytes()).unwrap()).unwrap();
            btree.insert(3, &Data::try_from("03".as_bytes()).unwrap()).unwrap();

            let result = btree.search(2);

            assert_eq!(result, Ok((1, 0)));
        }

        #[test]
        fn search_root_not_found() {
            let path = Path::new("test_files/search_root_not_found.json");
            let data_path = Path::new("test_files/search_root_not_found");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree.insert(1, &Data::try_from("01".as_bytes()).unwrap()).unwrap();
            btree.insert(3, &Data::try_from("03".as_bytes()).unwrap()).unwrap();
            btree.insert(4, &Data::try_from("04".as_bytes()).unwrap()).unwrap();

            let result = btree.search(2);

            assert_eq!(result, Err((1, 0)));
        }
    }

    mod insert_tests {
        use std::path::Path;

        use crate::{btree::BTree, data::Data};

        use super::*;
        #[test]
        fn insert_into_empty() {
            let path = Path::new("test_files/insert_into_empty.json");
            let data_path = Path::new("test_files/insert_into_empty");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree.insert(1, &Data::try_from("01".as_bytes()).unwrap()).unwrap();

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[{\"key\":1,\"data_id\":0}],\"children\":[null,null],\"is_leaf\":true,\"is_deleted\":false,\"id\":0}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_existing() {
            let path = Path::new("test_files/insert_existing.json");
            let data_path = Path::new("test_files/insert_existing");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree.insert(1, &Data::try_from("01".as_bytes()).unwrap()).unwrap();
            assert!(btree.insert(1, &Data::try_from("01".as_bytes()).unwrap()).is_err());

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[{\"key\":1,\"data_id\":0}],\"children\":[null,null],\"is_leaf\":true,\"is_deleted\":false,\"id\":0}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_existing_root() {
            let path = Path::new("test_files/insert_into_existing_root.json");
            let data_path = Path::new("test_files/insert_into_existing_root");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree.insert(1, &Data::try_from("01".as_bytes()).unwrap()).unwrap();
            btree.insert(3, &Data::try_from("02".as_bytes()).unwrap()).unwrap();
            btree.insert(0, &Data::try_from("03".as_bytes()).unwrap()).unwrap();
            btree.insert(2, &Data::try_from("04".as_bytes()).unwrap()).unwrap();

            let correct_btree = "[{\"parent_node_id\":null,\"keys\":[{\"key\":0,\"data_id\":2},{\"key\":1,\"data_id\":0},{\"key\":2,\"data_id\":3},{\"key\":3,\"data_id\":1}],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"is_deleted\":false,\"id\":0}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        /*
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
        use std::path::Path;

        use crate::btree::BTree;

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
        fn delete_merge(){
            let path = Path::new("test_files/delete_merge.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(0).unwrap();
            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(3).unwrap();
            btree.insert(4).unwrap();

            btree.delete(2).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[0,1,2,3],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[6,7,8],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[4],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
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
        use std::path::Path;

        use crate::btree::BTree;

        use super::*;

        #[test]
        fn update_same_node() {
            let path = Path::new("test_files/update_same_node.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(0).unwrap();
            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            btree.insert(6).unwrap();
            btree.insert(7).unwrap();

            btree.update(1, 3).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[0,2,3,4],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[6,7],\"children\":[null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[5],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_non_existent() {
            let path = Path::new("test_files/update_non_existent.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(0).unwrap();
            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            btree.insert(6).unwrap();
            btree.insert(7).unwrap();

            assert!(btree.update(3, 8).is_err());

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[0,1],\"children\":[null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[4,5,6,7],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[2],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_into_existing() {
            let path = Path::new("test_files/update_into_existing.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(0).unwrap();
            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            btree.insert(6).unwrap();
            btree.insert(7).unwrap();

            assert!(btree.update(1, 2).is_err());

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[0,1],\"children\":[null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[4,5,6,7],\"children\":[null,null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[2],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_change_node() {
            let path = Path::new("test_files/update_change_node.json");
            let mut btree = BTree::new(&path, 2);

            btree.insert(0).unwrap();
            btree.insert(1).unwrap();
            btree.insert(2).unwrap();
            btree.insert(4).unwrap();
            btree.insert(5).unwrap();
            btree.insert(6).unwrap();
            btree.insert(7).unwrap();

            btree.update(1, 8).unwrap();

            let correct_btree = "[{\"parent_node_id\":2,\"keys\":[0,2,4],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":0},{\"parent_node_id\":2,\"keys\":[6,7,8],\"children\":[null,null,null,null],\"is_leaf\":true,\"id\":1},{\"parent_node_id\":null,\"keys\":[5],\"children\":[0,1],\"is_leaf\":false,\"id\":2}]";
            let read_btree = fs::read_to_string(path).unwrap();

            assert_eq!(read_btree, correct_btree);
        }
*/
    }
}
