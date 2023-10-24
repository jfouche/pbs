mod db {
    use crate::{Database, ItemMaturity, Strategy};

    #[test]
    fn init_database() {
        assert!(Database::open(":memory:").is_ok());
    }

    #[test]
    fn add_items() {
        let db = Database::open(":memory:").unwrap();
        assert!(db.insert_item("PN1", "NAME1", Strategy::Make).is_ok());
        let items = db.items();
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(1, items.len());
    }

    #[test]
    fn add_childrens() {
        let db = Database::open(":memory:").unwrap();
        let item1 = db.insert_item("1", "PARENT", Strategy::Make).unwrap();
        let item2 = db.insert_item("11", "CHILD1", Strategy::Make).unwrap();
        let item3 = db.insert_item("12", "CHILD2", Strategy::Make).unwrap();
        db.add_child(item1.id(), item2.id(), 1).unwrap();
        db.add_child(item1.id(), item3.id(), 2).unwrap();
        let children = db.children(item1.id()).unwrap();
        assert_eq!(2, children.len());

        // can't add an already existing child
        assert!(db.add_child(item1.id(), item3.id(), 2).is_err());
    }

    #[test]
    fn add_same_pn() {
        let db = Database::open(":memory:").unwrap();
        let _ = db.insert_item("PN", "ITEM", Strategy::Make).unwrap();
        assert!(db.insert_item("PN", "ANOTHER", Strategy::Make).is_err());
    }

    #[test]
    fn config() {
        let db = Database::open(":memory:").unwrap();
        assert_eq!("".to_string(), db.read_config("key").unwrap());
        assert!(db.write_config("key", "value").is_ok());
        assert_eq!("value".to_string(), db.read_config("key").unwrap());
        let _ = db.write_config("key", "value 2");
        assert_eq!("value 2", db.read_config("key").unwrap());
    }

    #[test]
    fn search() {
        let db = Database::open(":memory:").unwrap();
        db.insert_item("00000001", "FIRST ITEM", Strategy::Make)
            .unwrap();
        db.insert_item("00000002", "SECOND ITEM", Strategy::Make)
            .unwrap();
        db.insert_item("00000003", "THIRD THING", Strategy::Make)
            .unwrap();
        db.insert_item("123.456", "BUY THING", Strategy::Make)
            .unwrap();
        db.insert_item("123.003", "OTHER BUY THING", Strategy::Make)
            .unwrap();
        db.insert_item("123.678", "THING 1003", Strategy::Make)
            .unwrap();

        let items = db.search("%000%").unwrap();
        assert_eq!(3, items.len());
        assert_eq!("00000001", items.get(0).unwrap().pn());
        assert_eq!("00000002", items.get(1).unwrap().pn());
        assert_eq!("00000003", items.get(2).unwrap().pn());

        let items = db.search("%003%").unwrap();
        assert_eq!(3, items.len());
        assert_eq!("00000003", items.get(0).unwrap().pn());
        assert_eq!("123.003", items.get(1).unwrap().pn());
        assert_eq!("123.678", items.get(2).unwrap().pn());
    }

    #[test]
    fn ensure_unique() {
        let db = Database::open(":memory:").unwrap();

        // test unique PN / version
        let res_1 = db.insert_item("PN", "NAME 1", Strategy::Buy);
        assert!(res_1.is_ok());
        assert!(db.insert_item("PN", "NAME 2", Strategy::Buy).is_err());

        // test unique parent/child
        let item1 = res_1.unwrap();
        let item2 = db.insert_item("PN2", "NAME 2", Strategy::Make).unwrap();
        assert!(db.add_child(item2.id(), item1.id(), 10).is_ok());
        assert!(db.add_child(item2.id(), item1.id(), 5).is_err());
    }
}

#[cfg(test)]
mod store {
    use crate::{Error, ItemMaturity, Store};

    #[test]
    fn release() {
        let store = Store::open(":memory:").expect("can't open store");
        let parent = store.make_item("PARENT").unwrap();
        let item1 = store.make_item("ITEM1").unwrap();
        let item2 = store.make_item("ITEM2").unwrap();
        let cots1 = store.buy_item("EXT.001", "COTS1").unwrap();
        let cots2 = store.buy_item("EXT.002", "COTS2").unwrap();

        store.add_child(item1.id(), cots1.id(), 1).unwrap();
        store.add_child(item2.id(), cots1.id(), 1).unwrap();
        store.add_child(item2.id(), cots2.id(), 2).unwrap();
        store.add_child(parent.id(), item1.id(), 1).unwrap();
        store.add_child(parent.id(), item2.id(), 1).unwrap();

        assert_eq!(
            Some(Error::CantReleaseItem),
            store.release(parent.id()).err()
        );

        assert!(store.release(item1.id()).is_ok());

        assert_eq!(
            Some(Error::CantReleaseItem),
            store.release(parent.id()).err()
        );

        assert!(store.release(item2.id()).is_ok());
        assert!(store.release(parent.id()).is_ok());
    }

    #[test]
    fn add_child() {
        let store = Store::open(":memory:").expect("can't open store");
        let parent = store.make_item("PARENT").unwrap();
        let item1 = store.make_item("ITEM1").unwrap();
        let item2 = store.make_item("ITEM2").unwrap();

        assert!(store.add_child(parent.id(), item1.id(), 1).is_ok());

        assert!(store.release(item1.id()).is_ok());
        assert!(store.release(parent.id()).is_ok());

        // parent is released, one can't add child anymore
        dbg!(store.item(parent.id()).unwrap());
        assert_eq!(
            Some(Error::CantAddChild),
            dbg!(store.add_child(parent.id(), item2.id(), 1)).err()
        );
    }

    #[test]
    fn make_obsolete() {
        let store = Store::open(":memory:").expect("can't open store");
        let parent = store.make_item("PARENT").unwrap();
        let item1 = store.make_item("ITEM1").unwrap();
        let item2 = store.make_item("ITEM2").unwrap();
        let cots1 = store.buy_item("EXT.001", "COTS1").unwrap();
        let cots2 = store.buy_item("EXT.002", "COTS2").unwrap();

        // parent           PARENT
        //  + item1         ITEM1     1
        //  |  + cots1      COTS1     1
        //  + item2         ITEM2     1
        //  |  + cots1      COTS1     1
        //  |  + cots2      COTS2     2

        store.add_child(item1.id(), cots1.id(), 1).unwrap();
        store.add_child(item2.id(), cots1.id(), 1).unwrap();
        store.add_child(item2.id(), cots2.id(), 2).unwrap();
        store.add_child(parent.id(), item1.id(), 1).unwrap();
        store.add_child(parent.id(), item2.id(), 1).unwrap();

        assert!(store.release(item1.id()).is_ok());
        assert!(store.release(item2.id()).is_ok());
        assert_eq!(
            ItemMaturity::Obsolete,
            store.make_obsolete(cots2.id()).unwrap().maturity()
        );
        assert_eq!(
            ItemMaturity::Obsolete,
            store.item(item2.id()).unwrap().maturity()
        );
        assert_eq!(
            ItemMaturity::InProgress,
            store.item(parent.id()).unwrap().maturity()
        );

        assert!(store.remove_child(parent.id(), item2.id()).is_ok());
        assert!(store.release(parent.id()).is_ok());

        assert_eq!(
            ItemMaturity::Obsolete,
            store.make_obsolete(cots1.id()).unwrap().maturity()
        );
        assert_eq!(
            ItemMaturity::Obsolete,
            store.item(item1.id()).unwrap().maturity()
        );
        assert_eq!(
            ItemMaturity::Obsolete,
            store.item(parent.id()).unwrap().maturity()
        );
    }
}
