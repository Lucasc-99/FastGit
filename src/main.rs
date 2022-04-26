use std::process::Command;

/*
    Fast Git (fgi):
    A basic cli script that concatenates git add and git commit
*/

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse Switch
    let switch = &args[1];

    match switch.as_str() {
        "commit" => {
            if args.len() < 4 {
                println!("Usage: fgi commit files <message>");
                return;
            }
            commit(&args[2..args.len() - 1], &args[args.len() - 1])
        }

        _ => {
            println!("fgi: error: unknown switch");
        }
    }
}

fn commit(files: &[String], commit_message: &String) {
    // Add
    let a_output = Command::new("git")
        .arg("add")
        .args(files)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&a_output.stdout));

    // Commit
    let c_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&c_output.stdout));
}
