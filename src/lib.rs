use anyhow::{Context, Result};
use serde_json::Map;
use serde_json::Value;
use similar::{ChangeTag, DiffOp, TextDiff};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::{diff, open_file};
    use anyhow::Result;
    use serde_json::Value;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn simple_no_diff() -> Result<()> {
        let file_path1 = PathBuf::from("./data/simple/test1.json");
        let file_path2 = PathBuf::from("./data/simple/test2.json");
        let file1 = open_file(file_path1)?;
        let file2 = open_file(file_path2)?;

        let v1: Value = serde_json::from_reader(file1)?;
        let v2: Value = serde_json::from_reader(file2)?;
        assert_eq!(diff(v1, v2, 3), "");
        Ok(())
    }

    #[test]
    fn simple_has_diff() -> Result<()> {
        let file2 = open_file(PathBuf::from("./data/simple/test2.json"))?;
        let file3 = open_file(PathBuf::from("./data/simple/test3.json"))?;
        let expected = fs::read_to_string("./data/simple/expected2_3.diff")?;

        let v1: Value = serde_json::from_reader(file2)?;
        let v2: Value = serde_json::from_reader(file3)?;
        assert_eq!(diff(v1, v2, 3), expected);
        Ok(())
    }

    #[test]
    fn deep_no_diff() -> Result<()> {
        let file_path1 = PathBuf::from("./data/deep/test1.json");
        let file_path2 = PathBuf::from("./data/deep/test2.json");
        let file1 = open_file(file_path1)?;
        let file2 = open_file(file_path2)?;

        let v1: Value = serde_json::from_reader(file1)?;
        let v2: Value = serde_json::from_reader(file2)?;
        assert_eq!(diff(v1, v2, 3), "");
        Ok(())
    }

    #[test]
    fn deep_has_diff() -> Result<()> {
        let file2 = open_file(PathBuf::from("./data/deep/test2.json"))?;
        let file3 = open_file(PathBuf::from("./data/deep/test3.json"))?;
        let expected = fs::read_to_string("./data/deep/expected2_3.diff")?;

        let v1: Value = serde_json::from_reader(file2)?;
        let v2: Value = serde_json::from_reader(file3)?;
        assert_eq!(diff(v1, v2, 3), expected);
        Ok(())
    }
}

pub fn normalize_from_file_path(file_path: PathBuf) -> Value {
    let file = File::open(file_path).unwrap();
    normalize_from_reader(file)
}

pub fn normalize_from_reader(file: File) -> Value {
    let v: Value = serde_json::from_reader(file).unwrap();
    normalize_value(v, true)
}

pub fn open_file(file_path: PathBuf) -> Result<File> {
    let file_path_str = file_path
        .to_str()
        .context("invalid path is given")?
        .to_string();
    File::open(file_path).context(format!("file not found: {}", file_path_str))
}

pub fn diff(v1: Value, v2: Value, unified: usize) -> String {
    let pretty_json1 = serde_json::to_string_pretty(&normalize_value(v1, true)).unwrap();
    let pretty_json2 = serde_json::to_string_pretty(&normalize_value(v2, true)).unwrap();
    let diff = TextDiff::from_lines(&pretty_json1, &pretty_json2);
    let mut ret_str = "".to_string();

    for diff_ops in diff.grouped_ops(unified) {
        for diff_op in diff_ops.iter() {
            let indices = match diff_op {
                DiffOp::Equal {
                    new_index,
                    old_index,
                    ..
                } => (old_index, new_index),
                DiffOp::Delete {
                    new_index,
                    old_index,
                    ..
                } => (old_index, new_index),
                DiffOp::Insert {
                    new_index,
                    old_index,
                    ..
                } => (old_index, new_index),
                DiffOp::Replace {
                    new_index,
                    old_index,
                    ..
                } => (old_index, new_index),
            };
            let mut equal_cnt = 0;
            for change in diff.iter_changes(diff_op) {
                let prefix = match change.tag() {
                    ChangeTag::Delete => format!("{}: - ", indices.0),
                    ChangeTag::Insert => format!("{}: + ", indices.1),
                    ChangeTag::Equal => {
                        let s = format!("{}:   ", indices.0 + equal_cnt);
                        equal_cnt += 1;
                        s
                    }
                };
                ret_str = format!("{}{}{}", ret_str, prefix, change);
            }
        }
        ret_str = format!("{}{}", ret_str, "----\n");
    }
    ret_str
}

fn generate_key(v: &Value) -> String {
    return match v {
        Value::Null => "__null__".to_string(),
        Value::Bool(bool_v) => {
            if *bool_v {
                "__true__".to_string()
            } else {
                "__false__".to_string()
            }
        }
        Value::Number(num) => num.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => arr
            .iter()
            .fold(String::new(), |s, av| s + &generate_key(av)),
        Value::Object(obj) => obj.iter().fold(String::new(), |s, (k, _)| s + k),
    };
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
                    let normalized_v = normalize_value(e, normalize_array);
                    m.insert(generate_key(&normalized_v), normalized_v);
                    m
                })
                .into_iter()
                .map(|(_k, v)| v)
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
