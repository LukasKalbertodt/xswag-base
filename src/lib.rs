pub mod code;
pub mod diag;

#[test]
fn it_works() {
    println!("{:?}", code::BytePos(7) - code::BytePos(4));
}
