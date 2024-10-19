use std::io::Error;

macro_rules! check_single_feature {
    ($($feature:literal),*) => {
        #[cfg(any(
            $(all(feature = $feature, $(feature = $feature),*)),*,
            not(any($(feature = $feature),*))
        ))]
        compile_error!("You can only enable one language feature at a time.");
    };
}
check_single_feature!(
    "bash",
    "c",
    "clojure",
    "cobol",
    "crystal",
    "dart",
    "elixir",
    "erlang",
    "fsharp",
    "fortran",
    "go",
    "groovy",
    "haskell",
    "java",
    "julia",
    "kotlin",
    "lua",
    "matlab",
    "nim",
    "nodejs",
    "objectivec",
    "perl",
    "php",
    "r",
    "ruby",
    "rust",
    "scala",
    "swift",
    "typescript",
    "vlang"
);
#[cfg(feature = "bash")]
pub mod bash;


pub mod utils;

#[cfg(feature = "c")]
pub mod c;

#[cfg(feature = "clojure")]
pub mod clojure;

#[cfg(feature = "cobol")]
pub mod cobol;

#[cfg(feature = "crystal")]
pub mod crystal;

#[cfg(feature = "dart")]
pub mod dart;

#[cfg(feature = "elixir")]
pub mod elixir;

#[cfg(feature = "erlang")]
pub mod erlang;

#[cfg(feature = "fsharp")]
pub mod f_sharp;

#[cfg(feature = "fortran")]
pub mod fortran;

#[cfg(feature = "go")]
pub mod go;

#[cfg(feature = "groovy")]
pub mod groovy;

#[cfg(feature = "haskell")]
pub mod haskell;

#[cfg(feature = "java")]
pub mod java;

#[cfg(feature = "julia")]
pub mod julia;

#[cfg(feature = "kotlin")]
pub mod kotlin;

#[cfg(feature = "lua")]
pub mod lua;

#[cfg(feature = "matlab")]
pub mod matlab;

#[cfg(feature = "nim")]
pub mod nim;
#[cfg(feature = "nodejs")]
pub mod nodejs;

#[cfg(feature = "objectivec")]
pub mod objective_c;

#[cfg(feature = "perl")]
pub mod perl;

#[cfg(feature = "php")]
pub mod php;

#[cfg(feature = "r")]
pub mod r;

#[cfg(feature = "ruby")]
pub mod ruby;

#[cfg(feature = "rust")]
pub mod rust;

#[cfg(feature = "scala")]
pub mod scala;

#[cfg(feature = "swift")]
pub mod swift;

#[cfg(feature = "typescript")]
pub mod typescript;

#[cfg(feature = "vlang")]
pub mod vlang;

pub fn main() -> Result<(), Error> {
    Err(Error::other("An unexpected error occurred. It seems you're trying something new! If this issue persists, we’d love your help. Please consider opening a Pull Request (PR) to help improve the project. Check out our contributing guidelines and submit your ideas here: https://github.com/otechdo/zuu/pulls."))
}
