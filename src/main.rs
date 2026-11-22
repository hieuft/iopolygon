use std::env;
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

fn write_test(name: &str, input: &str, output: &str, with_output: bool) -> std::io::Result<()> {
    fs::write(
        format!("./output/{name}.inp"),
        if with_output {
            format!("{input}\nTHISISSEPARATOR\n{output}")
        } else {
            String::from(input)
        },
    )
}

fn visit_dir(name: &str, dir: &Path, depth: usize, with_output: bool) -> std::io::Result<()> {
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
                println!("{indent}{name}/   {name}");
                let _ = visit_dir(&name, &path, depth + 1, with_output);
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
            let _ = write_test(&name, &input, &output, with_output);
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut with_output: bool = true;
    let start_path = Path::new("./data");

    if args.len() > 1 && &args[1] == "no-output" {
        with_output = false;
    }
    let _ = visit_dir("", start_path, 0, with_output);
    Ok(())
}
