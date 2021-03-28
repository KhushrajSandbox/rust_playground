pub struct Config<'q, 'c> {
    pub query: &'q str,
    pub contents: &'c str,
    pub is_case_sensitive: bool,
}

pub fn search<'c>(config: Config<'_, 'c>) -> Vec<&'c str> {
    let query = if config.is_case_sensitive {
        config.query.to_owned()
    } else {
        config.query.to_lowercase()
    };

    let mut results = Vec::new();
    for line in config.contents.lines() {
        if config.is_case_sensitive {
            if line.contains(query.as_str()) {
                results.push(line);
            }
        } else {
            if line.to_lowercase().contains(query.as_str()) {
                results.push(line);
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(Config {
                query,
                contents,
                is_case_sensitive: true
            })
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(Config {
                query,
                contents,
                is_case_sensitive: false
            })
        );
    }
}
