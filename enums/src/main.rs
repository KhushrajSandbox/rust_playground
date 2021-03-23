enum Language {
    C,
    Rust,
    JavaScript,
    TypeScript,
}

impl Language {
    fn name(&self) -> &str {
        match self {
            Language::C => "C",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Rust => "Rust",
        }
    }

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
    );

    let language_im_learning = Some(Language::Rust);

    match language_im_learning {
        Some(lang) => {
            println!("I'm currently learning: {}", lang.name());
            println!(
                "What do you think of the language I'm learning? -- {}",
                lang.react_to()
            )
        }
        None => println!("I'm not currently learning a language!"),
    }
}
