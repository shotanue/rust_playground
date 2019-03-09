extern crate sled;

use sled::Db;

fn main() {
    let dir_path = "test_storage";
    let tree = Db::start_default(dir_path).unwrap();

    let key = "hoge";
    let value = "abcdefg";

    // `API similar to a threadsafe BTreeMap<Vec<u8>, Vec<u8>>` とのこと。
    // tree.setのvalueのtypeも `type Value = Vec<u8>;`
    // を要求しているの渡すときに`as_bytes().to_vec()`を噛ませる
    tree.set(&key, value.as_bytes().to_vec())
        .expect("set fail");

    match tree.get(&key) {
        Ok(Some(res)) => {
            let converted: String = String::from_utf8(res.to_vec()).unwrap();
            assert_eq!(value,converted);
            println!("{}", converted); // abcdefg
        }
        _ => {}
    }
    tree.flush();
}
