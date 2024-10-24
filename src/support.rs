#[derive(PartialEq)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Go,
    D,
    Python,
    Php,
    Unknown,
}

#[derive(Default)]
pub struct Support {
    pub languages: Vec<Language>,
}
impl Support {
    pub fn new() -> Self {
        Self {
            languages: vec![
                Language::Rust,
                Language::D,
                Language::Go,
                Language::JavaScript,
                Language::TypeScript,
                Language::Php,
                Language::Python,
            ],
        }
    }
    pub fn all(self) -> Vec<String> {
        let mut data: Vec<String> = Vec::new();
        for language in self.languages {
            match language {
                Language::Rust => data.push(String::from("Rust")),
                Language::JavaScript => data.push(String::from("Javascript")),
                Language::TypeScript => data.push(String::from("TypeScript")),
                Language::Go => data.push(String::from("Go")),
                Language::D => data.push(String::from("D")),
                Language::Python => data.push(String::from("Python")),
                Language::Php => data.push(String::from("Php")),
                _ => continue,
            }
        }
        data
    }
}

impl Language {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "rust" => Self::Rust,
            "go" => Self::Go,
            "d" => Self::D,
            "js" => Self::JavaScript,
            "typescript" => Self::TypeScript,
            "python" => Self::Python,
            "php" => Self::Php,
            _ => Self::Unknown,
        }
    }
}
