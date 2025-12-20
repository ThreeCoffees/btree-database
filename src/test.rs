#[cfg(test)]
mod tests {
    use std::fs;

    mod search_tests {
        use std::path::Path;

        use crate::{
            btree::{BTree, InsertedData},
            data::Data,
            record::Record,
        };

        #[test]
        fn search_empty() {
            let path = Path::new("test_files/search_empty.json");
            let data_path = Path::new("test_files/search_empty");
            let mut btree = BTree::new(&path, &data_path, 3);

            let result = btree.search(1);

            assert_eq!(result, Err((0, 0)));

            btree.print();
        }

        #[test]
        fn search_root_find() {
            let path = Path::new("test_files/search_root_find.json");
            let data_path = Path::new("test_files/search_root_find");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("02".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("03".as_bytes()).unwrap()),
                )
                .unwrap();

            let result = btree.search(2);

            assert_eq!(result, Ok((Record::new(2, 1), 0)));
            btree.print();
        }

        #[test]
        fn search_root_not_found() {
            let path = Path::new("test_files/search_root_not_found.json");
            let data_path = Path::new("test_files/search_root_not_found");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("03".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("04".as_bytes()).unwrap()),
                )
                .unwrap();

            let result = btree.search(2);

            assert_eq!(result, Err((1, 0)));
            btree.print();
        }
    }

    mod insert_tests {
        use std::{fs::File, io::Read, path::Path};

        use crate::{
            btree::{BTree, InsertedData},
            data::Data,
        };

        #[test]
        fn insert_into_empty() {
            let path = Path::new("test_files/insert_into_empty.json");
            let data_path = Path::new("test_files/insert_into_empty");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_existing() {
            let path = Path::new("test_files/insert_existing.json");
            let data_path = Path::new("test_files/insert_existing");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            assert!(
                btree
                    .insert(
                        1,
                        InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap())
                    )
                    .is_err()
            );

            let correct_btree = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_existing_root() {
            let path = Path::new("test_files/insert_into_existing_root.json");
            let data_path = Path::new("test_files/insert_into_existing_root");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("02".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("03".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("04".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_left() {
            let path = Path::new("test_files/insert_into_full_root_left.json");
            let data_path = Path::new("test_files/insert_into_full_root_left");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    9,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    11,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0,
                0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 4, 0,
                0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_right() {
            let path = Path::new("test_files/insert_into_full_root_right.json");
            let data_path = Path::new("test_files/insert_into_full_root_right");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    9,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    11,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    10,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 6,
                0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_root_middle() {
            let path = Path::new("test_files/insert_into_full_root_middle.json");
            let data_path = Path::new("test_files/insert_into_full_root_middle");
            let mut btree = BTree::new(&path, &data_path, 3);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    9,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    11,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 4, 0,
                0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_leaf_split() {
            let path = Path::new("test_files/insert_into_full_leaf_split.json");
            let data_path = Path::new("test_files/insert_into_full_leaf_split");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    9,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    11,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    13,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    15,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    17,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    18,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 13,
                0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0,
                0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 18,
                0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn insert_into_full_leaf_compensation() {
            let path = Path::new("test_files/insert_into_full_leaf_compensation.json");
            let data_path = Path::new("test_files/insert_into_full_leaf_compensation");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    9,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    11,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    13,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    15,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .insert(
                    17,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    18,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    19,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    20,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 13,
                0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0,
                0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 18,
                0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0,
                0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }
    }

    mod delete_tests {
        use std::{fs::File, io::Read, path::Path};

        use crate::{
            btree::{BTree, InsertedData},
            data::Data,
        };

        #[test]
        fn delete_from_empty() {
            let path = Path::new("test_files/delete_from_empty.json");
            let data_path = Path::new("test_files/delete_from_empty");
            let mut btree = BTree::new(&path, &data_path, 2);

            assert!(btree.delete(0).is_err());

            let correct_btree: Vec<u8> = vec![];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_non_existent() {
            let path = Path::new("test_files/delete_non_existent.json");
            let data_path = Path::new("test_files/delete_non_existent");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            assert!(btree.delete(6).is_err());

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 5, 0,
                0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
                0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_from_root() {
            let path = Path::new("test_files/delete_from_root.json");
            let data_path = Path::new("test_files/delete_from_root");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree.delete(3).unwrap();

            let correct_btree = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_from_leaf() {
            let path = Path::new("test_files/delete_from_leaf.json");
            let data_path = Path::new("test_files/delete_from_leaf");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree.delete(2).unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 6, 0,
                0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
                0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_merge() {
            let path = Path::new("test_files/delete_merge.json");
            let data_path = Path::new("test_files/delete_merge");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree.delete(2).unwrap();

            let correct_btree = vec![
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            btree.print_all_nodes();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn delete_from_middle() {
            let path = Path::new("test_files/delete_from_middle.json");
            let data_path = Path::new("test_files/delete_from_middle");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    8,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            btree.delete(5).unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0,
                0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
                0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }
    }

    mod update_tests {
        use std::{fs::File, io::Read, path::Path};

        use crate::{
            btree::{BTree, InsertedData},
            data::Data,
        };

        #[test]
        fn update_same_node() {
            let path = Path::new("test_files/update_same_node.json");
            let data_path = Path::new("test_files/update_same_node");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("00".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("02".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("03".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("04".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("05".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("06".as_bytes()).unwrap()),
                )
                .unwrap();

            btree
                .update(
                    1,
                    3,
                    InsertedData::NewData(Data::try_from("13".as_bytes()).unwrap()),
                )
                .unwrap();

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
                0, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 7, 0,
                0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0,
                0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_non_existent() {
            let path = Path::new("test_files/update_non_existent.json");
            let data_path = Path::new("test_files/update_non_existent");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            assert!(btree.update(3, 8, InsertedData::None).is_err());

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 5, 0,
                0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0,
                0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
                0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_into_existing() {
            let path = Path::new("test_files/update_into_existing.json");
            let data_path = Path::new("test_files/update_into_existing");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            assert!(btree.update(1, 2, InsertedData::None).is_err());

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6, 0,
                0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
                0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_change_node() {
            let path = Path::new("test_files/update_change_node.json");
            let data_path = Path::new("test_files/update_change_node");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("00".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("02".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("03".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("04".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("05".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("06".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("07".as_bytes()).unwrap()),
                )
                .unwrap();

            assert!(btree.update(1, 8, InsertedData::None).is_ok());
            assert!(
                btree
                    .update(
                        3,
                        1,
                        InsertedData::NewData(
                            Data::try_from("change data and key".as_bytes()).unwrap()
                        )
                    )
                    .is_ok()
            );

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
                0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6, 0,
                0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0,
                0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
                0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }

        #[test]
        fn update_in_place() {
            let path = Path::new("test_files/update_in_place.json");
            let data_path = Path::new("test_files/update_in_place");
            let mut btree = BTree::new(&path, &data_path, 2);

            btree
                .insert(
                    0,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    1,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    2,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    3,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    4,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    5,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    6,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();
            btree
                .insert(
                    7,
                    InsertedData::NewData(Data::try_from("01".as_bytes()).unwrap()),
                )
                .unwrap();

            assert!(
                btree
                    .update(
                        1,
                        1,
                        InsertedData::NewData(Data::new(Some(Vec::from("after update"))))
                    )
                    .is_ok()
            );

            let correct_btree = vec![
                5, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 6, 0,
                0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0,
                0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];
            let mut file = File::open(path).unwrap();
            let mut read_btree = vec![];
            btree.nodes_file.write_cache();
            file.read_to_end(&mut read_btree).unwrap();

            btree.print();
            assert_eq!(read_btree, correct_btree);
        }
    }
}
