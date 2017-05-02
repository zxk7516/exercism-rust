use std::collections::BTreeMap;
pub fn transform(bt: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    let mut result: BTreeMap<String, i32> = BTreeMap::new();
    for (&k, ref v) in bt.iter(){
        for v_s in  v.iter() {
            result.insert( v_s.to_lowercase(), k);
        }
    }
    result
}