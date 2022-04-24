use std::process::Command;

fn main() {
    /*
        A basic cli script that concatenates git add and git commit
    */

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("Not enough arguments");
    } else {
        
        // Parse args
        let files = &args[1..args.len() - 1];
        let commit_message = &args[args.len() - 1];

        // Add 
        let output = Command::new("git")
        .arg("add")
        .args(files)
        .output()
        .expect("failed to execute process");
        
        // Commit
        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_message)
            .output()
            .expect("failed to execute process");
    }
}
