use std::io::Cursor;

use binrw::BinRead;
use jom::ClassFile;

#[test]
fn read() {
    let file = include_bytes!("HelloWorld.class");

    ClassFile::read(&mut Cursor::new(file)).unwrap();
}
