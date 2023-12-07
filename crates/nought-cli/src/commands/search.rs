use colored::Colorize;

pub(crate) fn _search(pkg_name: String) {
    for _ in 0..9 {
        println!(
            "{}", search_entry(
                "repo".to_string(),
                "pkg".to_string(),
                "v0.0.1".to_string(),
                "A simple description".to_string()
            )
        )
    }
}

fn search_entry(
    repo_name: String,
    pkg_name: String,
    pkg_version: String,
    description: String
) -> String {
    format!("{}/{}  {} \n {}",
            repo_name.blue(),
            pkg_name.truecolor(238, 130, 50),
            pkg_version,
            description
    )
}