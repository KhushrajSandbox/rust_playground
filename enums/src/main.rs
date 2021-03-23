enum Language {
    C,
    Rust,
    JavaScript,
    TypeScript,
}

impl Language {
    fn react_to(&self) -> &str {
        match self {
            Language::C => "🤮",
            Language::JavaScript => "👎",
            Language::TypeScript => "👍",
            Language::Rust => "😍",
        }
    }
}

fn main() {
    println!(
        concat!(
            "Rust: {}\n",
            "TypeScript: {}\n",
            "JavaScript: {}\n",
            "C: {}\n",
        ),
        Language::Rust.react_to(),
        Language::TypeScript.react_to(),
        Language::JavaScript.react_to(),
        Language::C.react_to()
    )
}
