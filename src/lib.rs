use serde_json::Map;
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use similar::{DiffOp, ChangeTag, TextDiff};

pub fn diff(v1: Value, v2: Value, unified: usize) -> String {
    let pretty_json1 = serde_json::to_string_pretty(&normalize_value(v1, true)).unwrap();
    let pretty_json2 = serde_json::to_string_pretty(&normalize_value(v2, true)).unwrap();
    let diff = TextDiff::from_lines(&pretty_json1, &pretty_json2);
    let mut ret_str = "".to_string();

    for diff_ops in diff.grouped_ops(unified) {
        for diff_op in diff_ops.iter() {
            let indices = match diff_op {
                DiffOp::Equal { new_index, old_index, .. } => { (old_index, new_index) }
                DiffOp::Delete { new_index, old_index, .. } => { (old_index, new_index) }
                DiffOp::Insert { new_index, old_index, .. } => { (old_index, new_index) }
                DiffOp::Replace { new_index, old_index, .. } => { (old_index, new_index) }
            };
            let mut equal_cnt = 0;
            for change in diff.iter_changes(diff_op) {
                let prefix = match change.tag() {
                    ChangeTag::Delete => format!("{}: - ", indices.0),
                    ChangeTag::Insert => format!("{}: + ", indices.1),
                    ChangeTag::Equal => {
                        let s = format!("{}:   ", indices.0+equal_cnt);
                        equal_cnt += 1;
                        s
                    },
                };
                ret_str = format!("{}{}{}", ret_str, prefix, change);
            }
        }
        ret_str = format!("{}{}", ret_str, "----\n");
    }
    ret_str
}

pub fn normalize_value(v: Value, normalize_array: bool) -> Value {
    match v {
        Value::Array(av) => {
            if !normalize_array {
                return Value::from(av);
            }
            let new_v: Vec<Value> = av
                .into_iter()
                .fold(BTreeMap::new(), |mut m, e| {
                    let mut hasher = DefaultHasher::new();
                    e.to_string().hash(&mut hasher);
                    m.insert(hasher.finish(), e);
                    m
                })
                .into_iter()
                .map(|(_k, v)| v)
                .map(|v| normalize_value(v, normalize_array))
                .collect();
            Value::from(new_v)
        }
        Value::Object(ov) => {
            let new_obj = ov.into_iter().fold(Map::new(), |mut m, (k, v)| {
                m.insert(k, normalize_value(v, normalize_array));
                m
            });
            Value::from(new_obj)
        }
        _ => return v,
    }
}
