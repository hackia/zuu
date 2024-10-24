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
}
