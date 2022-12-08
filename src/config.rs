use std::{
    fs::{File,OpenOptions},
    io::{BufReader, Read ,Write},
    path::Path,
};

pub async fn get_id() -> String {
    let f = File::open(Path::new("/etc/funny.conf")).unwrap();
    let mut content = String::new();
    let mut buf = BufReader::new(f);
    let _ = buf.read_to_string(&mut content).unwrap();

    content
}

pub async fn set_id(id: &str) {
    let mut _file = OpenOptions::new()
        .write(true)
        .truncate(true) // 清空文件
        .open("/etc/funny.conf").unwrap();
    _file.write_all(id.as_bytes()).unwrap();
}
