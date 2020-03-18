#[test]
fn iter_demo(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next().unwrap(),&1);
    assert_eq!(v1_iter.next().unwrap(),&2);
    assert_eq!(v1_iter.next().unwrap(),&3);
    assert_eq!(v1_iter.next(),None);
}

#[test]
fn iter_sum(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum();
    assert_eq!(total,6);
}