use super::*;

#[test]
fn test_integer() {
    let empty = RcList::new();

    let s = RcList::new_append(1u8, &empty);
    let s = RcList::new_append(2, &s);
    let s1 = RcList::new_append(3, &s);
    let s2 = RcList::new_append(4, &s);

    let vempty : Vec<u8> = empty.iter().cloned().collect();
    let v1 : Vec<u8> = s1.iter().cloned().collect();
    let v2 : Vec<u8> = s2.iter().cloned().collect();

    assert_eq!(vempty, []);
    assert_eq!(v1, [3, 2, 1]);
    assert_eq!(v2, [4, 2, 1]);
}

#[test]
fn test_str() {
    let empty = RcList::new();

    let s = RcList::new_append("1", &empty);
    let s = RcList::new_append("2", &s);
    let s1 = RcList::new_append("3", &s);
    let s2 = RcList::new_append("4", &s);

    let vempty : Vec<&str> = empty.iter().cloned().collect();
    let v1 : Vec<&str> = s1.iter().cloned().collect();
    let v2 : Vec<&str> = s2.iter().cloned().collect();

    let empty_vec : Vec<&str> = vec!();
    assert_eq!(vempty, empty_vec);
    assert_eq!(v1, ["3", "2", "1"]);
    assert_eq!(v2, ["4", "2", "1"]);
}

#[test]
fn test_string() {
    let empty = RcList::new();

    let s = RcList::new_append("1".to_string(), &empty);
    let s = RcList::new_append("2".to_string(), &s);
    let s1 = RcList::new_append("3".to_string(), &s);
    let s2 = RcList::new_append("4".to_string(), &s);

    let vempty : Vec<String> = empty.iter().cloned().collect();
    let v1 : Vec<String> = s1.iter().cloned().collect();
    let v2 : Vec<String> = s2.iter().cloned().collect();

    let empty_vec : Vec<String> = vec!();
    assert_eq!(vempty, empty_vec);
    assert_eq!(v1, ["3", "2", "1"]);
    assert_eq!(v2, ["4", "2", "1"]);
}


#[test]
fn test_weak() {

    let (s1, s2) = {
        let empty = RcList::new();

        let s = RcList::new_append(1u8, &empty);
        let s = RcList::new_append(2, &s);
        let s1 = RcList::new_append_weak(3, &s);
        let s2 = RcList::new_append_weak(4, &s);

        (s1, s2)
    };

    let v1 : Vec<u8> = s1.iter().cloned().collect();
    let v2 : Vec<u8> = s2.iter().cloned().collect();

    assert_eq!(v1, [3]);
    assert_eq!(v2, [4]);
}



