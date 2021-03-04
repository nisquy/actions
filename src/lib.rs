pub fn one() -> u32 {
    1
}

pub fn two() -> u32 {
    2
}

pub fn three() -> u32 {
    3
}

#[test]
fn unit() {
    assert_eq!(one() + two(), three());
}
