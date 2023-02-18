use jom::ClassFile;

#[test]
fn read() {
    let file = include_bytes!("HelloWorld.class");

    ClassFile::read(file).unwrap();
}
