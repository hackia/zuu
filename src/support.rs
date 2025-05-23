use std::fmt::Display;

#[derive(PartialEq)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Go,
    D,
    Python,
    Php,
    Java,
    Kotlin,
    Swift,
    Scala,
    Ruby,
    Perl,
    R,
    Haskell,
    Lua,
    ObjectiveC,
    C,
    Cpp, // C++
    Nim,
    Crystal,
    FSharp, // F#
    Dart,
    Elixir,
    Bash,
    Zsh,
    Fish,
    Unknown,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Go => write!(f, "Go"),
            Language::D => write!(f, "D"),
            Language::Python => write!(f, "Python"),
            Language::Php => write!(f, "Php"),
            Language::Java => write!(f, "Java"),
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Swift => write!(f, "Swift"),
            Language::Scala => write!(f, "Scala"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Perl => write!(f, "Perl"),
            Language::R => write!(f, "R"),
            Language::Haskell => write!(f, "Haskell"),
            Language::Lua => write!(f, "Variant1"),
            Language::ObjectiveC => write!(f, "ObjectiveC"),
            Language::C => write!(f, "C"),
            Language::Cpp => write!(f, "Cpp"),
            Language::Nim => write!(f, "Nim"),
            Language::Crystal => write!(f, "Crystal"),
            Language::FSharp => write!(f, "FSharp"),
            Language::Dart => write!(f, "Dart"),
            Language::Elixir => write!(f, "Elixir"),
            Language::Bash => write!(f, "Bash"),
            Language::Zsh => write!(f, "Zsh"),
            Language::Fish => write!(f, "Fish"),
            Language::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Default)]
pub struct Support {
    pub languages: Vec<Language>,
}
impl Support {
    #[must_use]
    pub fn new() -> Self {
        Self {
            languages: vec![
                Language::Rust,
                Language::JavaScript,
                Language::TypeScript,
                Language::Go,
                Language::D,
                Language::Python,
                Language::Php,
                Language::Java,
                Language::Kotlin,
                Language::Swift,
                Language::Scala,
                Language::Ruby,
                Language::Perl,
                Language::R,
                Language::Haskell,
                Language::Lua,
                Language::ObjectiveC,
                Language::C,
                Language::Cpp,
                Language::Nim,
                Language::Crystal,
                Language::FSharp,
                Language::Dart,
                Language::Elixir,
                Language::Bash,
                Language::Zsh,
                Language::Fish,
            ],
        }
    }

    #[must_use]
    #[doc = "Get all supported languages"]
    pub fn all(self) -> Vec<String> {
        let mut data: Vec<String> = Vec::new();
        for language in &self.languages {
            match language {
                Language::Rust => data.push(String::from("Rust")),
                Language::JavaScript => data.push(String::from("Javascript")),
                Language::TypeScript => data.push(String::from("TypeScript")),
                Language::Go => data.push(String::from("Go")),
                Language::D => data.push(String::from("D")),
                Language::Python => data.push(String::from("Python")),
                Language::Php => data.push(String::from("Php")),
                Language::Java => data.push(String::from("Java")),
                Language::Kotlin => data.push(String::from("Kotlin")),
                Language::Swift => data.push(String::from("Swift")),
                Language::Scala => data.push(String::from("Scala")),
                Language::Ruby => data.push(String::from("Ruby")),
                Language::Perl => data.push(String::from("Perl")),
                Language::R => data.push(String::from("R")),
                Language::Haskell => data.push(String::from("Haskell")),
                Language::Lua => data.push(String::from("Lua")),
                Language::ObjectiveC => data.push(String::from("ObjectiveC")),
                Language::C => data.push(String::from("C")),
                Language::Cpp => data.push(String::from("Cpp")),
                Language::Nim => data.push(String::from("Nim")),
                Language::Crystal => data.push(String::from("Crystal")),
                Language::FSharp => data.push(String::from("Fsharp")),
                Language::Dart => data.push(String::from("Dart")),
                Language::Elixir => data.push(String::from("Elixir")),
                Language::Bash => data.push(String::from("Bash")),
                Language::Zsh => data.push(String::from("Zsh")),
                Language::Fish => data.push(String::from("Fish")),
                Language::Unknown => continue,
            }
        }
        data
    }
    #[must_use]
    #[doc = "Get all supported language"]
    pub fn supported(self) -> Vec<Language> {
        self.languages
    }
}
