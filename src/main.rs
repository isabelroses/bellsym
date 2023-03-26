use serde_json;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json = loadjson(&args[1]);
    let moves = json.get("move").unwrap().as_object().unwrap();
    for (key, value) in moves {
        let src = Path::new(key);
        let dst = Path::new(value.as_str().unwrap());
        if src.is_dir() {
            let x = move_dir_recursive(src, dst);
            if x.is_err() {
                println!("Unable to move directory: {}", x.err().unwrap());
            }
        } else {
            let x = fs::rename(src, dst);
            if x.is_err() {
                println!("Unable to move file: {}", x.err().unwrap());
            }
        }
    }
    let symlink = json.get("symlink").unwrap().as_object().unwrap();
    for (key, value) in symlink {
        std::os::unix::fs::symlink(key, value.as_str().unwrap()).expect("Unable to create symlink");
    }
    println!("Done!");
}

fn loadjson(x: &str) -> serde_json::Value {
    let file = std::fs::read_to_string(x).expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not well-formatted");
    return json;
}

fn move_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;

        for entry in src.read_dir()? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                move_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::rename(&src_path, &dst_path)?;
            }
        }
    } else {
        fs::rename(src, dst)?;
    }

    fs::remove_dir_all(src)?;
    Ok(())
}
