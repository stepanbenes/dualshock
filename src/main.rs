use std::fs::File;
use std::io::Read;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let filename = "C:/temp/test.txt".to_string();
    loop {
        let data = get_file_as_byte_vec(&filename);
        println!("{:?}", data);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
