use std::process::Command;

fn main() {
    // Get the git commit hash
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    let git_hash = match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    };

    // Get the git branch name
    let branch_output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output();

    let git_branch = match branch_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    };

    // TODO: update the dates in LICENSE to always reflect current year
    // Get current year with 'date +%Y'
    let date_output = Command::new("date")
        .args(&["+%Y"])
        .output();

    let date_year = match date_output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => "Error: Check build.rs date_year".to_string(),
    };

    let date_start = "2024"; // date that seemsgood_guild repo was created with license.


    // Set environment variables for use in the code
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=CURRENT_YEAR={}-{}", date_start, date_year);
    
    // Re-run if .git/HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
