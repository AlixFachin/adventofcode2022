use serde_json::json;
use serde_json::to_string;
use serde_json::Value;
use std::cmp::Ordering;
use std::fs;
use std::panic;

const FILEPATH: &str = "src/day13/input.txt";

fn min(i1: usize, i2: usize) -> usize {
    if i1 <= i2 {
        return i1;
    }
    return i2;
}

fn compare_values(v1: &Value, v2: &Value) -> Ordering {
    println!(" Comparing {:?}\n  and\n{:?}", v1, v2);
    if v1.is_number() && v2.is_number() {
        println!("  Returning {}", v1.as_i64() <= v2.as_i64());
        return v1.as_i64().cmp(&v2.as_i64());
    }
    if v1.is_array() && v2.is_array() {
        let a1 = v1.as_array().unwrap();
        let a2 = v2.as_array().unwrap();
        for i in 0..min(a1.len(), a2.len()) {
            match compare_values(&a1[i], &a2[i]) {
                Ordering::Equal => {
                    // Equal -> have to look further
                }
                other => return other,
            }
        }
        return a1.len().cmp(&a2.len());
    }
    if v1.is_array() && !v2.is_array() {
        return compare_values(v1, &json!(vec![v2.as_i64()]));
    }
    if !v1.is_array() && v2.is_array() {
        return compare_values(&json!(vec![v1.as_i64()]), v2);
    }

    panic!("Unexpected type in comparison!");
}

#[test]
fn test_compare() {
    fn compare_messages(msg1: &str, msg2: &str) -> bool {
        let token1_json = format!("{{\"message\":{} \n}}", msg1);
        let v1: Value = serde_json::from_str(token1_json.as_str()).unwrap();
        let packet1 = &v1["message"];

        let token2_json: String = format!("{{\"message\":{} \n}}", msg2);
        let v2: Value = serde_json::from_str(token2_json.as_str()).unwrap();
        let packet2 = &v2["message"];

        return compare_values(&packet1, &packet2) == Ordering::Less;
    }

    // Normal test - array same size
    assert!(compare_messages("[1,1,3,1,1]", "[1,1,5,1,1]"));

    // Both have integer values
    assert!(compare_messages("32", "64"));
    assert!(!compare_messages("64", "4"));

    // Array 1 is smaller
    assert!(compare_messages("[1,2,3]", "[1,2,3,4,5]"));
    assert!(!compare_messages("[9,2,3]", "[1,2,3,4,5]"));
    assert!(compare_messages("1", "[1,2,3,4,5]"));
    assert!(!compare_messages("9", "[1,2,3,4,5]"));

    // Array 1 is biggger
    assert!(!compare_messages("[1,2,3,4,5]", "[1,2,3]"));
    assert!(compare_messages("[1,1,3,4,5]", "[1,2,3]"));
    assert!(!compare_messages("[3,4,5]", "1"));
    assert!(compare_messages("[3,4,5]", "6"));
    assert!(!compare_messages("[3,4,5]", "3"));

    // Cases with empty array
    assert!(compare_messages("[]", "[1,2,3,4,5]"));
    assert!(!compare_messages("[1,2,3]", "[]"));
    assert!(compare_messages("[1,2,[]]", "[1,2,3]"));
    assert!(!compare_messages("[1,2,3]", "[1,2,[]]"));

    // Cases with sub-Array
    assert!(compare_messages("[[1,1,1],2,3,4]", "[[1,1,1],4,5,6]"));
    assert!(!compare_messages("[[1,1,1],2]", "[[1,1,1],1,5,6]"));
    assert!(!compare_messages("[[1,1,1],2,3,4]", "[[1,1,1],2,3,4]"));
}

pub fn solve() {
    let contents = fs::read_to_string(FILEPATH).expect("Should have been able to read the file");
    let mut tokens_array: Vec<&str> = contents.split("\n").filter(|x| !x.is_empty()).collect();
    tokens_array.push("[[2]]");
    tokens_array.push("[[6]]");

    let mut packets_array: Vec<Value> = tokens_array
        .iter()
        .map(|s| {
            serde_json::from_str::<Value>(format!("{{\"message\":{} \n}}", s).as_str()).unwrap()
        })
        .collect();

    packets_array.sort_by(|a, b| compare_values(&a["message"], &b["message"]));

    let mut i;
    let mut index_2 = 0;
    let mut index_6 = 0;
    i = 0;
    for packet in packets_array {
        if packet["message"].to_string() == "[[2]]" {
            index_2 = i + 1;
        }
        if packet["message"].to_string() == "[[6]]" {
            index_6 = i + 1;
        }

        i += 1;
    }

    println!(
        "Solution of question 2:\n index2 = {}, index6 = {}, solution={}",
        index_2,
        index_6,
        index_2 * index_6
    );
}
