use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

//file for reading
#[test]
fn read() {
    //ensure file.txt is available otherwise panic will happen - error is not being handled here
    let mut f = File::open("file.txt").unwrap();
    //create a buffer on stack
    let mut buf: [u8; 20] = [0; 20];
    f.read(&mut buf).unwrap();
    print!("{:?}", buf);
}

//file for writing
#[test]
fn write() {
    let mut f = File::create("file.txt").unwrap();
    f.write(b"Hello World").unwrap();
}

//file open for any option
#[test]
fn open_for_any_option() {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .append(true)
        .open("file.txt")
        .unwrap();
    f.write(b" Hello Other World").unwrap();
}

//buffered read/write in std lib
#[test]
fn buffered_read() {
    let f = File::open("file.txt").unwrap();
    let mut buf_reader = BufReader::new(f);
    let mut buf: [u8; 100] = [0; 100];
    buf_reader.read(&mut buf).unwrap();
    print!("{:?}", buf);
}

#[test]
fn buffered_write() {
    let mut f = File::create("file.txt").unwrap();
    let mut buf_writer = BufWriter::new(f);
    buf_writer.write(b"Hello World").unwrap();
}
