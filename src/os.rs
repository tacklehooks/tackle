use std::{env, fs};

/// Test if the target executable exists in path.
pub fn is_program_in_path(program: &str) -> bool {
    let delimiter = if cfg!(windows) { ";" } else { ":" };
    if let Ok(path) = env::var("PATH") {
        for p in path.split(delimiter) {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::os::is_program_in_path;

    #[test]
    fn test_is_program_in_path() {
        assert!(is_program_in_path("git"));
        assert!(!is_program_in_path("not_a_program"));
    }
}
