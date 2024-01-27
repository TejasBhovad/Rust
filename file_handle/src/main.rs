use std::fs;
fn main() {
    create_dir();
    create_file();
    read_file();
    // remove_dir();
    // remove_dir_all()
}
fn create_dir() {
    let path = "./data";
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("Directory Exists!");
        return;
    }
    let res = fs::create_dir(path);
    if res.is_ok() {
        println!("Created Directory")
    } else {
        println!("Error: {:?}", res.err())
    }
}
fn create_file() {
    let path = "./data/file_1.md";
    let path_1 = "./data/file_2.md";
    let text = "File contents";
    let text_1 = "File contents 1";

    let _res = std::fs::write(path, text);
    let _res = std::fs::write(path_1, text_1);

    // let _ = std::fs::remove_file(path);
}

fn remove_dir() {
    let path = "./data";
    let res = std::fs::remove_dir(path);
    if res.is_ok() {
        println!("Deleted Empty directory");
    } else {
        println!("Error: {:?}", res.err())
    }
}

fn remove_dir_all() {
    let path = "./data";
    let res = std::fs::remove_dir_all(path);
    if res.is_ok() {
        println!("Deleted directory");
    } else {
        println!("Error: {:?}", res.err())
    }
}

fn read_file() {
    let file_to_read = "./data/file_1.md";
    let convert_bytes_to_string = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        return a;
    };
    let read_result = std::fs::read(file_to_read);
    if read_result.is_ok() {
        println!("Data:");
        println!(
            "{}",
            read_result
                .ok()
                .unwrap()
                .iter()
                .fold(String::from(""), convert_bytes_to_string)
        );
    } else {
        println!("Error: {:?}", read_result.err())
    }
}
