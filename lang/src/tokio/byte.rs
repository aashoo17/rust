use bytes::{BufMut, Bytes, BytesMut};

//TODO: explain the logic of using bytes crate
//[bytes docs](https://docs.rs/bytes/1.0.1/bytes/)
#[tokio::main]
#[test]
async fn bytes_uses() {
    let mut a = BytesMut::new();
    a.put_i32(10);
    println!("{:?}", a);

    tokio::spawn(async move {
        a.put_i32(10);
        println!("{:?}", a);
    });
}
