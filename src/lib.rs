pub mod code;

#[test]
fn it_works() {
    println!("{:?}", code::BytePos(7) - code::BytePos(4));
}
