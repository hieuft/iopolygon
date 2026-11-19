use std::fs;
use std::path::Path;

const EXCLUDED: [&str; 2] = ["target", "Settings.cfg"];

fn is_excluded(name: &str) -> bool {
    name.starts_with(".") || EXCLUDED.contains(&name)
}

fn read_file(path: &Path) -> String {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents
}

fn is_input(name: &str) -> bool {
    return name.to_lowercase().contains(".i");
}

fn is_output(name: &str) -> bool {
    return name.to_lowercase().contains(".o");
}

fn write_test(test_index: usize, input: &str, output: &str) -> std::io::Result<()> {
    fs::write(
        format!("./output/{test_index}"),
        format!("{input}\nTHISISSEPARATOR\n{output}"),
    )
}

fn visit_dir(dir: &Path, depth: usize, mut test_index: usize) -> std::io::Result<()> {
    if dir.is_dir() {
        let indent = "  ".repeat(depth);
        let mut input: String = String::from("");
        let mut output: String = String::from("");

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();

            if is_excluded(&name) {
                continue;
            }

            if path.is_dir() {
                test_index += 1;
                println!("{indent}{name}/   {test_index}");
                let _ = visit_dir(&path, depth + 1, test_index);
            } else {
                if is_input(&name) {
                    input = read_file(&path);
                } else if is_output(&name) {
                    output = read_file(&path);
                }
                println!("{indent}{name}");
            }
        }

        if input.len() > 0 || output.len() > 0 {
            let _ = write_test(test_index, &input, &output);
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let start_path = Path::new("./data");
    let _ = visit_dir(start_path, 0, 0);
    Ok(())
}
