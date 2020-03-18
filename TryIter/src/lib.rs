#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next().unwrap(), &1);
    assert_eq!(v1_iter.next().unwrap(), &2);
    assert_eq!(v1_iter.next().unwrap(), &3);
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[cfg(test)]
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
#[cfg(test)]
fn shoes_in_my_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|shoe| shoe.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let in_my_size: Vec<&Shoe>;
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    println!("{:?}", shoes);

    in_my_size = shoes_in_my_size(&shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            &Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            &Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
