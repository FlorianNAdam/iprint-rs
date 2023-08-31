use crate::iformat;

fn level1() -> String {
    iformat!("level1") + "\n" + &level2() + "\n" + &iformat!("level1")
}

fn level2() -> String {
    iformat!("level2")
}

#[test]
fn iformat_test() {
    assert_eq!(
        "level1\n    level2\nlevel1",
        level1());
}
