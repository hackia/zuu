pub mod ask;
pub mod output;
pub mod runner;
pub mod support;
#[doc = "All checkup tasks to execute for R with success and failure messages"]
pub const R_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the R project structure", // Description
        "R CMD check .",                      // Command to check the R project structure
        "Project structure is valid",         // Success message
        "Invalid project structure",          // Failure message
    ),
    (
        "Checking R package dependencies",          // Description
        "Rscript -e 'devtools::check_deps()'", // Command to check if the required dependencies are installed (via devtools)
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",     // Failure message
    ),
    (
        "Checking for R security vulnerabilities", // Description
        "Rscript -e 'rscans::scan_project()'", // Command to check for security issues (requires rscans)
        "No security vulnerabilities found",   // Success message
        "Security vulnerabilities detected",   // Failure message
    ),
    (
        "Running all R tests",           // Description
        "Rscript -e 'devtools::test()'", // Command to run unit tests (requires devtools and testthat)
        "All tests passed",              // Success message
        "Some tests failed",             // Failure message
    ),
    (
        "Validating R code formatting",     // Description
        "Rscript -e 'styler::style_pkg()'", // Command to check code formatting (requires styler)
        "Code is correctly formatted",      // Success message
        "Code formatting issues detected",  // Failure message
    ),
    (
        "Generating R project documentation",   // Description
        "Rscript -e 'devtools::document()'", // Command to generate project documentation (requires roxygen2)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation",  // Failure message
    ),
    (
        "Checking for outdated R dependencies",     // Description
        "Rscript -e 'devtools::update_packages()'", // Command to check for outdated dependencies
        "No outdated dependencies",                 // Success message
        "Outdated dependencies detected",           // Failure message
    ),
    (
        "Linting the R source code",          // Description
        "Rscript -e 'lintr::lint_package()'", // Command to lint the R code (requires lintr)
        "Code linting passed",                // Success message
        "Code linting issues detected",       // Failure message
    ),
    (
        "Checking for unused R dependencies",        // Description
        "Rscript -e 'devtools::clean_vignettes()'", // Command to clean unused dependencies or build artifacts
        "No unused dependencies",                   // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Perl with success and failure messages"]
pub const PERL_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Perl project structure",     // Description
        "perl Makefile.PL && make",                  // Command to check the Perl project structure
        "Project structure is valid",                // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking Perl project dependencies",       // Description
        "cpan -T", // Command to check if the required dependencies are installed
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for Perl security vulnerabilities", // Description
        "perlcritic --brutal .", // Command to check for security issues and coding standards (requires Perl::Critic)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities or coding issues detected", // Failure message
    ),
    (
        "Running all Perl tests", // Description
        "prove -l",               // Command to run tests (using `prove`)
        "All tests passed",       // Success message
        "Some tests failed",      // Failure message
    ),
    (
        "Validating Perl code formatting", // Description
        "perltidy -b **/*.pl **/*.pm", // Command to check Perl code formatting (requires Perltidy)
        "Code is correctly formatted", // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Perl project documentation", // Description
        "pod2html lib/*.pm > docs/index.html", // Command to generate project documentation from POD
        "Documentation generated successfully", // Success message
        "Failed to generate documentation",    // Failure message
    ),
    (
        "Checking for outdated Perl dependencies", // Description
        "cpan-outdated -p", // Command to check for outdated dependencies (requires cpan-outdated)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Perl source code", // Description
        "perlcritic --stern .",         // Command to lint the Perl code (using Perl::Critic)
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
    (
        "Checking for unused Perl dependencies",     // Description
        "make clean", // Command to clean unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for Swift with success and failure messages"]
pub const SWIFT_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Swift project structure",    // Description
        "swift build", // Command to build and check the project structure
        "Project structure is valid", // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking Swift project dependencies",      // Description
        "swift package resolve",                    // Command to resolve and verify dependencies
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for Swift security vulnerabilities", // Description
        "swiftlint analyze", // Command to analyze security issues (using SwiftLint)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Swift tests", // Description
        "swift test",              // Command to run tests
        "All tests passed",        // Success message
        "Some tests failed",       // Failure message
    ),
    (
        "Validating Swift code formatting", // Description
        "swiftformat --lint .",             // Command to check code formatting (using SwiftFormat)
        "Code is correctly formatted",      // Success message
        "Code formatting issues detected",  // Failure message
    ),
    (
        "Generating Swift project documentation", // Description
        "swift doc generate",                     // Command to generate project documentation
        "Documentation generated successfully",   // Success message
        "Failed to generate documentation",       // Failure message
    ),
    (
        "Checking for outdated Swift dependencies", // Description
        "swift package show-dependencies --format json", // Command to check for outdated dependencies
        "No outdated dependencies",                      // Success message
        "Outdated dependencies detected",                // Failure message
    ),
    (
        "Linting the Swift source code", // Description
        "swiftlint",                     // Command to lint the Swift code (using SwiftLint)
        "Code linting passed",           // Success message
        "Code linting issues detected",  // Failure message
    ),
    (
        "Checking for unused Swift dependencies",    // Description
        "swift package clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Scala with success and failure messages"]
pub const SCALA_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Scala project structure",    // Description
        "sbt compile", // Command to compile and check the project structure
        "Project structure is valid", // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking Scala project dependencies",      // Description
        "sbt update",                               // Command to update and verify dependencies
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for Scala security vulnerabilities", // Description
        "sbt dependencyCheckAnalyze", // Command to analyze security vulnerabilities (using sbt-dependency-check plugin)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Scala tests", // Description
        "sbt test",                // Command to run tests
        "All tests passed",        // Success message
        "Some tests failed",       // Failure message
    ),
    (
        "Validating Scala code formatting", // Description
        "scalafmt --test",                  // Command to check code formatting (using Scalafmt)
        "Code is correctly formatted",      // Success message
        "Code formatting issues detected",  // Failure message
    ),
    (
        "Generating Scala project documentation", // Description
        "sbt doc",                                // Command to generate project documentation
        "Documentation generated successfully",   // Success message
        "Failed to generate documentation",       // Failure message
    ),
    (
        "Checking for outdated Scala dependencies", // Description
        "sbt dependencyUpdates", // Command to check for outdated dependencies (using sbt-updates plugin)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Scala source code", // Description
        "scalafix --test",               // Command to lint the Scala code (using Scalafix)
        "Code linting passed",           // Success message
        "Code linting issues detected",  // Failure message
    ),
    (
        "Checking for unused Scala dependencies",    // Description
        "sbt clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Java with success and failure messages"]
pub const JAVA_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Java project structure",     // Description
        "gradle build --dry-run",                    // Command to validate the project structure
        "Project structure is valid",                // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking Java project dependencies",       // Description
        "gradle dependencies",                      // Command to check project dependencies
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for Java security vulnerabilities", // Description
        "gradle dependencyCheckAnalyze", // Command to check for vulnerabilities (requires OWASP dependency-check plugin)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Java tests", // Description
        "gradle test",            // Command to run all tests
        "All tests passed",       // Success message
        "Some tests failed",      // Failure message
    ),
    (
        "Validating Java code formatting", // Description
        "gradle checkstyleMain",           // Command to check code formatting (requires Checkstyle)
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Java project documentation", // Description
        "gradle javadoc",                        // Command to generate project documentation
        "Documentation generated successfully",  // Success message
        "Failed to generate documentation",      // Failure message
    ),
    (
        "Checking for outdated Java dependencies", // Description
        "gradle dependencyUpdates", // Command to check for outdated dependencies (requires Gradle Versions plugin)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Java source code", // Description
        "gradle spotbugsMain",          // Command to lint the code (requires SpotBugs)
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
    (
        "Checking for unused Java dependencies",     // Description
        "gradle clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for Kotlin with success and failure messages"]
pub const KOTLIN_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Kotlin project structure",   // Description
        "gradle build --dry-run",                    // Command to validate the project structure
        "Project structure is valid",                // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking Kotlin project dependencies",     // Description
        "gradle dependencies",                      // Command to check project dependencies
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for Kotlin security vulnerabilities", // Description
        "gradle dependencyCheckAnalyze", // Command to check for vulnerabilities (requires OWASP dependency-check plugin)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Kotlin tests", // Description
        "gradle test",              // Command to run all tests
        "All tests passed",         // Success message
        "Some tests failed",        // Failure message
    ),
    (
        "Validating Kotlin code formatting", // Description
        "gradle ktlintCheck",                // Command to check code formatting (requires Ktlint)
        "Code is correctly formatted",       // Success message
        "Code formatting issues detected",   // Failure message
    ),
    (
        "Generating Kotlin project documentation", // Description
        "gradle dokka", // Command to generate project documentation (requires Dokka)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Kotlin dependencies", // Description
        "gradle dependencyUpdates", // Command to check for outdated dependencies (requires Gradle Versions plugin)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Kotlin source code", // Description
        "gradle spotbugsMain",            // Command to lint the code (requires SpotBugs)
        "Code linting passed",            // Success message
        "Code linting issues detected",   // Failure message
    ),
    (
        "Checking for unused Kotlin dependencies",   // Description
        "gradle clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Dart with success and failure messages"]
pub const DART_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Dart project structure", // Description
        "dart pub get", // Command to validate the project structure and fetch dependencies
        "Project structure is valid", // Success message
        "Invalid project structure or failed to fetch dependencies", // Failure message
    ),
    (
        "Checking Dart project dependencies", // Description
        "dart pub outdated",                  // Command to check for outdated dependencies
        "All dependencies are up to date",    // Success message
        "Outdated dependencies detected",     // Failure message
    ),
    (
        "Checking for Dart security vulnerabilities", // Description
        "dart pub audit", // Command to audit for vulnerabilities (in newer Dart SDKs)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Dart tests", // Description
        "dart test",              // Command to run tests
        "All tests passed",       // Success message
        "Some tests failed",      // Failure message
    ),
    (
        "Validating Dart code formatting",                   // Description
        "dart format --output=none --set-exit-if-changed .", // Command to check code formatting
        "Code is correctly formatted",                       // Success message
        "Code formatting issues detected",                   // Failure message
    ),
    (
        "Generating Dart project documentation", // Description
        "dart doc",                              // Command to generate project documentation
        "Documentation generated successfully",  // Success message
        "Failed to generate documentation",      // Failure message
    ),
    (
        "Checking for outdated Dart dependencies", // Description
        "dart pub outdated",                       // Command to check for outdated dependencies
        "No outdated dependencies",                // Success message
        "Outdated dependencies detected",          // Failure message
    ),
    (
        "Linting the Dart source code", // Description
        "dart analyze",                 // Command to lint the code
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
    (
        "Cleaning unused Dart dependencies",         // Description
        "dart pub clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for Fish with success and failure messages"]
pub const FISH_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Fish script structure", // Description
        "fish -n ./*.fish",                     // Command to validate Fish script syntax
        "Fish script structure is valid",       // Success message
        "Fish script contains syntax errors",   // Failure message
    ),
    (
        "Checking Fish script dependencies", // Description
        "fish -c 'type -a'", // Command to check if commands used in the script are available
        "All dependencies are available", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for Fish script security vulnerabilities", // Description
        "shellcheck ./*.fish", // Command to check for security issues (using ShellCheck)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Fish script tests", // Description
        "fish ./test.fish",              // Command to run tests (assuming there's a test file)
        "All tests passed",              // Success message
        "Some tests failed",             // Failure message
    ),
    (
        "Validating Fish script formatting",      // Description
        "fish_indent -c ./*.fish",                // Command to check Fish script formatting
        "Fish script is correctly formatted",     // Success message
        "Fish script formatting issues detected", // Failure message
    ),
    (
        "Generating Fish script documentation", // Description
        "generate_docs_fish ./*.fish", // Custom command to generate documentation (if applicable)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Fish script dependencies", // Description
        "fish ./update_check.fish", // Command to check for outdated dependencies (assumed custom script)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Fish script",             // Description
        "shellcheck ./*.fish",                 // Command to lint the Fish script
        "Fish script linting passed",          // Success message
        "Fish script linting issues detected", // Failure message
    ),
    (
        "Cleaning unused Fish script dependencies", // Description
        "fish ./clean.fish", // Command to clean unused dependencies (custom script)
        "No unused dependencies", // Success message
        "Unused dependencies detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for Zsh with success and failure messages"]
pub const ZSH_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Zsh script structure", // Description
        "zsh -n ./*.zsh",                      // Command to validate Zsh script syntax
        "Zsh script structure is valid",       // Success message
        "Zsh script contains syntax errors",   // Failure message
    ),
    (
        "Checking Zsh script dependencies", // Description
        "zsh -c 'whence -v'", // Command to check if commands used in the script are available
        "All dependencies are available", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for Zsh script security vulnerabilities", // Description
        "shellcheck ./*.zsh", // Command to check for security issues (using ShellCheck)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Zsh script tests", // Description
        "zsh ./test.zsh",               // Command to run tests (assuming there's a test file)
        "All tests passed",             // Success message
        "Some tests failed",            // Failure message
    ),
    (
        "Validating Zsh script formatting",      // Description
        "shellcheck ./*.zsh",                    // Command to check Zsh script formatting
        "Zsh script is correctly formatted",     // Success message
        "Zsh script formatting issues detected", // Failure message
    ),
    (
        "Generating Zsh script documentation",  // Description
        "generate_docs_zsh ./*.zsh", // Custom command to generate documentation (if applicable)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Zsh script dependencies", // Description
        "zsh ./update_check.zsh", // Command to check for outdated dependencies (assumed custom script)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Zsh script",             // Description
        "shellcheck ./*.zsh",                 // Command to lint the Zsh script
        "Zsh script linting passed",          // Success message
        "Zsh script linting issues detected", // Failure message
    ),
    (
        "Cleaning unused Zsh script dependencies", // Description
        "zsh ./clean.zsh", // Command to clean unused dependencies (custom script)
        "No unused dependencies", // Success message
        "Unused dependencies detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for Bash with success and failure messages"]
pub const BASH_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Bash script structure", // Description
        "bash -n ./*.sh",                       // Command to validate Bash script syntax
        "Bash script structure is valid",       // Success message
        "Bash script contains syntax errors",   // Failure message
    ),
    (
        "Checking Bash script dependencies", // Description
        "bash -c 'type -a'", // Command to check if commands used in the script are available
        "All dependencies are available", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for Bash script security vulnerabilities", // Description
        "shellcheck ./*.sh", // Command to check for security issues (using ShellCheck)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Bash script tests", // Description
        "bash ./test.sh",                // Command to run tests (assuming there's a test file)
        "All tests passed",              // Success message
        "Some tests failed",             // Failure message
    ),
    (
        "Validating Bash script formatting",      // Description
        "shellcheck ./*.sh",                      // Command to check Bash script formatting
        "Bash script is correctly formatted",     // Success message
        "Bash script formatting issues detected", // Failure message
    ),
    (
        "Generating Bash script documentation", // Description
        "generate_docs_bash ./*.sh", // Custom command to generate documentation (if applicable)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Bash script dependencies", // Description
        "bash ./update_check.sh", // Command to check for outdated dependencies (assumed custom script)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Bash script",             // Description
        "shellcheck ./*.sh",                   // Command to lint the Bash script
        "Bash script linting passed",          // Success message
        "Bash script linting issues detected", // Failure message
    ),
    (
        "Cleaning unused Bash script dependencies",  // Description
        "bash ./clean.sh", // Command to clean unused dependencies (custom script)
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for Crystal with success and failure messages"]
pub const CRYSTAL_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Crystal project structure",  // Description
        "crystal build --no-codegen",                // Command to validate the project structure
        "Project structure is valid",                // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking Crystal project dependencies",    // Description
        "shards list",                              // Command to check dependencies (using Shards)
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for Crystal security vulnerabilities", // Description
        "crystal deps audit", // Command to check for vulnerabilities (requires external tool)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Crystal tests", // Description
        "crystal spec",              // Command to run tests
        "All tests passed",          // Success message
        "Some tests failed",         // Failure message
    ),
    (
        "Validating Crystal code formatting", // Description
        "crystal tool format --check",        // Command to check code formatting
        "Code is correctly formatted",        // Success message
        "Code formatting issues detected",    // Failure message
    ),
    (
        "Generating Crystal project documentation", // Description
        "crystal docs",                             // Command to generate project documentation
        "Documentation generated successfully",     // Success message
        "Failed to generate documentation",         // Failure message
    ),
    (
        "Checking for outdated Crystal dependencies", // Description
        "shards outdated",                            // Command to check for outdated dependencies
        "No outdated dependencies",                   // Success message
        "Outdated dependencies detected",             // Failure message
    ),
    (
        "Linting the Crystal source code", // Description
        "crystal tool format --check",     // Command to lint the code
        "Code linting passed",             // Success message
        "Code linting issues detected",    // Failure message
    ),
    (
        "Cleaning unused Crystal dependencies",      // Description
        "shards prune", // Command to clean unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];
#[doc = "All checkup tasks to execute for F# with success and failure messages"]
pub const FSHARP_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the F# project structure",       // Description
        "dotnet build --no-restore",                 // Command to validate project structure
        "Project structure is valid",                // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking F# project dependencies",         // Description
        "dotnet restore",                           // Command to check dependencies
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for F# security vulnerabilities", // Description
        "dotnet list package --vulnerable",         // Command to check for vulnerabilities
        "No security vulnerabilities found",        // Success message
        "Security vulnerabilities detected",        // Failure message
    ),
    (
        "Running all F# tests", // Description
        "dotnet test",          // Command to run tests
        "All tests passed",     // Success message
        "Some tests failed",    // Failure message
    ),
    (
        "Validating F# code formatting",   // Description
        "fantomas . --check",              // Command to check code formatting (using Fantomas)
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating F# project documentation",  // Description
        "dotnet fsdocs build",                  // Command to generate documentation
        "Documentation generated successfully", // Success message
        "Failed to generate documentation",     // Failure message
    ),
    (
        "Checking for outdated F# dependencies", // Description
        "dotnet outdated",                       // Command to check for outdated dependencies
        "No outdated dependencies",              // Success message
        "Outdated dependencies detected",        // Failure message
    ),
    (
        "Linting the F# source code",   // Description
        "dotnet fsharp lint",           // Command to lint the code
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
    (
        "Cleaning unused F# dependencies",           // Description
        "dotnet clean", // Command to clean unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Nim with success and failure messages"]
pub const NIM_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Nim project structure",         // Description
        "nim check", // Command to validate project structure and check for issues
        "Project structure is valid", // Success message
        "Project structure is invalid or issues found", // Failure message
    ),
    (
        "Checking Nim project dependencies",        // Description
        "nimble install",                           // Command to check and install dependencies
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies",          // Failure message
    ),
    (
        "Checking for Nim security vulnerabilities", // Description
        "nim check --threads:on", // Command to check for vulnerabilities and threading issues
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities or threading issues detected", // Failure message
    ),
    (
        "Running all Nim tests", // Description
        "nimble test",           // Command to run all tests
        "All tests passed",      // Success message
        "Some tests failed",     // Failure message
    ),
    (
        "Validating Nim code formatting",  // Description
        "nimble fmt --check",              // Command to check code formatting
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Nim project documentation", // Description
        "nim doc ./*.nim",                      // Command to generate project documentation
        "Documentation generated successfully", // Success message
        "Failed to generate documentation",     // Failure message
    ),
    (
        "Checking for outdated Nim dependencies", // Description
        "nimble outdated",                        // Command to check for outdated dependencies
        "No outdated dependencies",               // Success message
        "Outdated dependencies detected",         // Failure message
    ),
    (
        "Linting the Nim source code",         // Description
        "nim check --styleCheck:hint ./*.nim", // Command to lint the code and check for issues
        "Code linting passed",                 // Success message
        "Code linting issues detected",        // Failure message
    ),
    (
        "Cleaning unused Nim dependencies",          // Description
        "nimble clean", // Command to clean unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Objective-C with success and failure messages"]
pub const OBJC_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Objective-C project structure", // Description
        "xcodebuild clean", // Command to validate the project structure by cleaning the build
        "Project structure is valid", // Success message
        "Project structure is invalid or cleaning failed", // Failure message
    ),
    (
        "Checking Objective-C project dependencies", // Description
        "pod install", // Command to check dependencies (using CocoaPods)
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for Objective-C security vulnerabilities", // Description
        "clang --analyze ./*.m", // Command to check for vulnerabilities (using Clang Static Analyzer)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Objective-C tests",         // Description
        "xcodebuild test -scheme <scheme-name>", // Command to run tests
        "All tests passed",                      // Success message
        "Some tests failed",                     // Failure message
    ),
    (
        "Validating Objective-C code formatting", // Description
        "clang-format -style=file -i ./*.m",      // Command to check and format code
        "Code is correctly formatted",            // Success message
        "Code formatting issues detected",        // Failure message
    ),
    (
        "Generating Objective-C project documentation", // Description
        "appledoc .", // Command to generate project documentation (requires Appledoc)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Objective-C dependencies", // Description
        "pod outdated", // Command to check for outdated dependencies (using CocoaPods)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Objective-C source code", // Description
        "clang-tidy ./*.m",                    // Command to lint the code (using Clang-Tidy)
        "Code linting passed",                 // Success message
        "Code linting issues detected",        // Failure message
    ),
    (
        "Cleaning unused Objective-C dependencies",  // Description
        "pod deintegrate && pod clean", // Command to clean unused dependencies (using CocoaPods)
        "No unused dependencies",       // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Lua with success and failure messages"]
pub const LUA_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Lua project structure", // Description
        "luac -p ./*.lua",                      // Command to validate Lua script syntax
        "Lua script structure is valid",        // Success message
        "Lua script contains syntax errors",    // Failure message
    ),
    (
        "Checking Lua project dependencies",        // Description
        "luarocks list", // Command to check dependencies (using LuaRocks)
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for Lua security vulnerabilities", // Description
        "luacheck ./*.lua", // Command to check for vulnerabilities (using Luacheck)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Lua tests", // Description
        "busted",                // Command to run tests (using Busted)
        "All tests passed",      // Success message
        "Some tests failed",     // Failure message
    ),
    (
        "Validating Lua code formatting",  // Description
        "luacheck ./*.lua",                // Command to check code formatting
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Lua project documentation", // Description
        "ldoc .", // Command to generate project documentation (using LDoc)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Lua dependencies", // Description
        "luarocks list --outdated",               // Command to check for outdated dependencies
        "No outdated dependencies",               // Success message
        "Outdated dependencies detected",         // Failure message
    ),
    (
        "Linting the Lua source code",  // Description
        "luacheck ./*.lua",             // Command to lint the code
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
    (
        "Cleaning unused Lua dependencies", // Description
        "luarocks purge", // Command to clean unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Elixir with success and failure messages"]
pub const ELIXIR_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Elixir project structure", // Description
        "mix deps.get", // Command to validate the project structure and fetch dependencies
        "Project structure is valid", // Success message
        "Invalid project structure or failed to fetch dependencies", // Failure message
    ),
    (
        "Checking Elixir project dependencies", // Description
        "mix hex.outdated",                     // Command to check for outdated dependencies
        "All dependencies are up to date",      // Success message
        "Outdated dependencies detected",       // Failure message
    ),
    (
        "Checking for Elixir security vulnerabilities", // Description
        "mix audit", // Command to audit dependencies for vulnerabilities (using mix_audit)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Elixir tests", // Description
        "mix test",                 // Command to run tests
        "All tests passed",         // Success message
        "Some tests failed",        // Failure message
    ),
    (
        "Validating Elixir code formatting", // Description
        "mix format --check-formatted",      // Command to check code formatting
        "Code is correctly formatted",       // Success message
        "Code formatting issues detected",   // Failure message
    ),
    (
        "Generating Elixir project documentation", // Description
        "mix docs", // Command to generate project documentation (using ExDoc)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Elixir dependencies", // Description
        "mix hex.outdated",                          // Command to check for outdated dependencies
        "No outdated dependencies",                  // Success message
        "Outdated dependencies detected",            // Failure message
    ),
    (
        "Linting the Elixir source code", // Description
        "mix credo",                      // Command to lint the code (using Credo)
        "Code linting passed",            // Success message
        "Code linting issues detected",   // Failure message
    ),
    (
        "Cleaning unused Elixir dependencies",       // Description
        "mix deps.clean --unused", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies",  // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Rust with success and failure messages"]
pub const RUST_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Rust project structure", // Description
        "cargo verify-project",                  // Command to validate the project structure
        "Project structure is valid",            // Success message
        "Invalid project structure",             // Failure message
    ),
    (
        "Checking Rust project dependencies", // Description
        "cargo check",                        // Command to check build dependencies
        "No issues with build dependencies",  // Success message
        "Build dependency issues found",      // Failure message
    ),
    (
        "Checking for Rust build dependencies", // Description
        "cargo check",                          // Command to check dependencies without building
        "Build dependencies are valid",         // Success message
        "Invalid build dependencies",           // Failure message
    ),
    (
        "Scanning Rust project for security vulnerabilities", // Description
        "cargo audit", // Command to audit dependencies for vulnerabilities
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Rust tests", // Description
        "cargo test",             // Command to run unit tests
        "All tests passed",       // Success message
        "Some tests failed",      // Failure message
    ),
    (
        "Validating Rust code formatting", // Description
        "cargo fmt --check",               // Command to check code formatting
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Rust project documentation", // Description
        "cargo doc --no-deps",                   // Command to generate project documentation
        "Documentation generated successfully",  // Success message
        "Failed to generate documentation",      // Failure message
    ),
    (
        "Checking for outdated Rust dependencies", // Description
        "cargo outdated",                          // Command to check for outdated dependencies
        "No outdated dependencies",                // Success message
        "Outdated dependencies detected",          // Failure message
    ),
    (
        "Linting the Rust source code", // Description
        "cargo clippy -- -D warnings",  // Command to lint the Rust code
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Node.js with success and failure messages"]
pub const NODEJS_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Node.js project structure", // Description
        "npm run check-structure", // Command to check project structure (requires a custom script in package.json)
        "Project structure is valid", // Success message
        "Invalid project structure", // Failure message
    ),
    (
        "Checking Node.js project dependencies",   // Description
        "npm install",                             // Command to install and check dependencies
        "Dependencies are correctly installed",    // Success message
        "Dependency installation issues detected", // Failure message
    ),
    (
        "Checking for Node.js security vulnerabilities", // Description
        "npm audit", // Command to check for security vulnerabilities
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Node.js tests", // Description
        "npm test", // Command to run tests (configured via package.json, typically using jest or mocha)
        "All tests passed", // Success message
        "Some tests failed", // Failure message
    ),
    (
        "Validating Node.js code formatting", // Description
        "npm run format:check", // Command to check code formatting (usually using prettier)
        "Code is correctly formatted", // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Node.js project documentation", // Description
        "npm run generate-docs", // Command to generate project documentation (typically using jsdoc)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Node.js dependencies", // Description
        "npm outdated",                               // Command to check for outdated dependencies
        "No outdated dependencies",                   // Success message
        "Outdated dependencies detected",             // Failure message
    ),
    (
        "Linting the Node.js source code", // Description
        "npm run lint",                    // Command to run linter (using ESLint or similar)
        "Code linting passed",             // Success message
        "Code linting issues detected",    // Failure message
    ),
    (
        "Checking for unused Node.js dependencies", // Description
        "npm prune",                                // Command to remove unused dependencies
        "No unused dependencies",                   // Success message
        "Unused dependencies detected",             // Failure message
    ),
];

#[doc = "All checkup tasks to execute for PHP with success and failure messages"]
pub const PHP_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the PHP project structure", // Description
        "composer validate",                    // Command to validate the composer.json structure
        "Project structure is valid",           // Success message
        "Invalid project structure",            // Failure message
    ),
    (
        "Verifying PHP project licenses", // Description
        "composer licenses",              // Command to check licenses for dependencies
        "No license issues found",        // Success message
        "License issues detected",        // Failure message
    ),
    (
        "Checking PHP build dependencies",                   // Description
        "composer check-platform-reqs", // Command to check that dependencies match the platform requirements
        "All dependencies are compatible with the platform", // Success message
        "Dependency compatibility issues found", // Failure message
    ),
    (
        "Scanning PHP project for security vulnerabilities", // Description
        "composer audit", // Command to audit dependencies for vulnerabilities
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all PHP tests",    // Description
        "composer run-script test", // Command to run unit tests (defined in composer.json scripts)
        "All tests passed",         // Success message
        "Some tests failed",        // Failure message
    ),
    (
        "Validating PHP code formatting",  // Description
        "composer run-script fmt",         // Command to check code formatting (PSR-12, for example)
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating PHP project documentation", // Description
        "composer run-script doc", // Command to generate project documentation (using tools like phpDocumentor)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated PHP dependencies", // Description
        "composer outdated",                      // Command to check for outdated dependencies
        "No outdated dependencies",               // Success message
        "Outdated dependencies detected",         // Failure message
    ),
    (
        "Linting the PHP source code",  // Description
        "composer run-script lint",     // Command to run linter (using PHPStan or similar)
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for D with success and failure messages"]
pub const D_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the D project structure", // Description
        "dub describe",                       // Command to validate the project structure
        "Project structure is valid",         // Success message
        "Invalid project structure",          // Failure message
    ),
    (
        "Verifying D project licenses", // Description
        "dub fetch --licenses",         // Command to fetch and verify licenses for dependencies
        "No license issues found",      // Success message
        "License issues detected",      // Failure message
    ),
    (
        "Checking D build dependencies", // Description
        "dub upgrade",                   // Command to check and upgrade dependencies if necessary
        "Dependencies are up to date",   // Success message
        "Dependency issues found",       // Failure message
    ),
    (
        "Scanning D project for security vulnerabilities", // Description
        "dub audit", // Command to audit the project for vulnerabilities (or external tool)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all D tests", // Description
        "dub test",            // Command to run the unit tests in the D project
        "All tests passed",    // Success message
        "Some tests failed",   // Failure message
    ),
    (
        "Validating D code formatting",    // Description
        "dfmt --check", // Command to check if the D code is properly formatted (requires dfmt installed)
        "Code is correctly formatted", // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating D project documentation",   // Description
        "dub build --build=docs", // Command to generate documentation for the D project
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated D dependencies", // Description
        "dub outdated",                         // Command to check for outdated dependencies
        "No outdated dependencies",             // Success message
        "Outdated dependencies detected",       // Failure message
    ),
    (
        "Linting the D source code",    // Description
        "dscanner --styleCheck",        // Command to lint the D code (requires dscanner installed)
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Haskell with success and failure messages"]
pub const HASKELL_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Haskell project structure", // Description
        "stack build --dry-run", // Command to validate the project structure and dependencies (Stack-based)
        "Project structure is valid", // Success message
        "Invalid project structure", // Failure message
    ),
    (
        "Checking Haskell project dependencies", // Description
        "stack solver",                          // Command to check for dependency issues
        "No dependency issues found",            // Success message
        "Dependency issues detected",            // Failure message
    ),
    (
        "Checking for Haskell security vulnerabilities", // Description
        "cabal audit", // Command to audit Haskell dependencies for vulnerabilities (requires `cabal` with a custom tool)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Haskell tests", // Description
        "stack test",                // Command to run unit tests
        "All tests passed",          // Success message
        "Some tests failed",         // Failure message
    ),
    (
        "Validating Haskell code formatting", // Description
        "hindent .", // Command to check code formatting (requires `hindent`)
        "Code is correctly formatted", // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Haskell project documentation", // Description
        "stack haddock",                            // Command to generate project documentation
        "Documentation generated successfully",     // Success message
        "Failed to generate documentation",         // Failure message
    ),
    (
        "Checking for outdated Haskell dependencies", // Description
        "stack ls dependencies --outdated",           // Command to check for outdated dependencies
        "No outdated dependencies",                   // Success message
        "Outdated dependencies detected",             // Failure message
    ),
    (
        "Linting the Haskell source code", // Description
        "hlint .",                         // Command to run Haskell linter (requires `hlint`)
        "Code linting passed",             // Success message
        "Code linting issues detected",    // Failure message
    ),
    (
        "Checking for unused Haskell dependencies", // Description
        "stack clean --full",                       // Command to clean unused dependencies
        "No unused dependencies",                   // Success message
        "Unused dependencies detected",             // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Ruby with success and failure messages"]
pub const RUBY_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Ruby project structure", // Description
        "bundle check", // Command to check if dependencies are satisfied (via Bundler)
        "Project structure is valid", // Success message
        "Invalid project structure or dependency issues found", // Failure message
    ),
    (
        "Checking Ruby project dependencies",      // Description
        "bundle install", // Command to install and check dependencies (via Bundler)
        "Dependencies are correctly installed", // Success message
        "Dependency installation issues detected", // Failure message
    ),
    (
        "Checking for Ruby security vulnerabilities", // Description
        "bundle audit", // Command to check for vulnerabilities in the Gemfile (via `bundle-audit`)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Ruby tests", // Description
        "bundle exec rspec",      // Command to run tests using RSpec
        "All tests passed",       // Success message
        "Some tests failed",      // Failure message
    ),
    (
        "Validating Ruby code formatting",     // Description
        "bundle exec rubocop --format simple", // Command to check Ruby code formatting (via `rubocop`)
        "Code is correctly formatted",         // Success message
        "Code formatting issues detected",     // Failure message
    ),
    (
        "Generating Ruby project documentation", // Description
        "yard doc", // Command to generate project documentation (via `yard`)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated Ruby dependencies", // Description
        "bundle outdated", // Command to check for outdated dependencies (via Bundler)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the Ruby source code", // Description
        "bundle exec rubocop",          // Command to run RuboCop for linting
        "Code linting passed",          // Success message
        "Code linting issues detected", // Failure message
    ),
    (
        "Checking for unused Ruby dependencies", // Description
        "bundle clean", // Command to remove unused dependencies (via Bundler)
        "No unused dependencies", // Success message
        "Unused dependencies detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for C with success and failure messages"]
pub const C_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the C project structure",        // Description
        "make clean && make", // Command to build and check the project structure
        "Project structure is valid", // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking C project dependencies",          // Description
        "pkg-config --validate", // Command to check if required dependencies are installed (using pkg-config)
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for C security vulnerabilities",    // Description
        "cppcheck --enable=all --error-exitcode=1 .", // Command to check for security and coding issues (using cppcheck)
        "No security vulnerabilities found",          // Success message
        "Security vulnerabilities or coding issues detected", // Failure message
    ),
    (
        "Running all C tests", // Description
        "make test", // Command to run tests (requires a test suite defined in the Makefile)
        "All tests passed", // Success message
        "Some tests failed", // Failure message
    ),
    (
        "Validating C code formatting",                  // Description
        "clang-format --dry-run --Werror **/*.c **/*.h", // Command to check code formatting (using clang-format)
        "Code is correctly formatted",                   // Success message
        "Code formatting issues detected",               // Failure message
    ),
    (
        "Generating C project documentation",   // Description
        "doxygen Doxyfile", // Command to generate project documentation (using Doxygen)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated C dependencies", // Description
        "make outdated", // Command to check for outdated dependencies (requires custom implementation)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the C source code",                    // Description
        "cppcheck --enable=style --error-exitcode=1 .", // Command to lint the C code (using cppcheck)
        "Code linting passed",                          // Success message
        "Code linting issues detected",                 // Failure message
    ),
    (
        "Checking for unused C dependencies",        // Description
        "make clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for C++ with success and failure messages"]
pub const CPP_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the C++ project structure",      // Description
        "make clean && make", // Command to build and check the project structure
        "Project structure is valid", // Success message
        "Invalid project structure or build failed", // Failure message
    ),
    (
        "Checking C++ project dependencies",        // Description
        "pkg-config --validate", // Command to check if required dependencies are installed (using pkg-config)
        "All dependencies are correctly installed", // Success message
        "Missing or invalid dependencies", // Failure message
    ),
    (
        "Checking for C++ security vulnerabilities", // Description
        "cppcheck --enable=all --error-exitcode=1 .", // Command to check for security and coding issues (using cppcheck)
        "No security vulnerabilities found",          // Success message
        "Security vulnerabilities or coding issues detected", // Failure message
    ),
    (
        "Running all C++ tests", // Description
        "make test", // Command to run tests (requires a test suite defined in the Makefile)
        "All tests passed", // Success message
        "Some tests failed", // Failure message
    ),
    (
        "Validating C++ code formatting",                    // Description
        "clang-format --dry-run --Werror **/*.cpp **/*.hpp", // Command to check code formatting (using clang-format)
        "Code is correctly formatted",                       // Success message
        "Code formatting issues detected",                   // Failure message
    ),
    (
        "Generating C++ project documentation", // Description
        "doxygen Doxyfile", // Command to generate project documentation (using Doxygen)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation", // Failure message
    ),
    (
        "Checking for outdated C++ dependencies", // Description
        "make outdated", // Command to check for outdated dependencies (requires custom implementation)
        "No outdated dependencies", // Success message
        "Outdated dependencies detected", // Failure message
    ),
    (
        "Linting the C++ source code",                  // Description
        "cppcheck --enable=style --error-exitcode=1 .", // Command to lint the C++ code (using cppcheck)
        "Code linting passed",                          // Success message
        "Code linting issues detected",                 // Failure message
    ),
    (
        "Checking for unused C++ dependencies",      // Description
        "make clean", // Command to clean up unused dependencies or build artifacts
        "No unused dependencies", // Success message
        "Unused dependencies or artifacts detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Go with success and failure messages"]
pub const GO_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Go project structure", // Description
        "go mod verify", // Command to verify the Go project structure and dependencies
        "Project structure is valid", // Success message
        "Invalid project structure", // Failure message
    ),
    (
        "Verifying Go project dependencies", // Description
        "go mod tidy", // Command to tidy up the Go module and ensure dependencies are correct
        "All dependencies are correct", // Success message
        "Issues with project dependencies", // Failure message
    ),
    (
        "Checking for Go module inconsistencies",   // Description
        "go mod verify", // Command to verify dependencies match the checksum
        "No inconsistencies found in dependencies", // Success message
        "Inconsistent dependencies found", // Failure message
    ),
    (
        "Scanning Go project for security vulnerabilities", // Description
        "go list -m all | go run golang.org/x/vuln/cmd/govulncheck", // Command to check for vulnerabilities (requires govulncheck installed)
        "No security vulnerabilities found",                         // Success message
        "Security vulnerabilities detected",                         // Failure message
    ),
    (
        "Running all Go tests", // Description
        "go test ./...",        // Command to run Go tests
        "All tests passed",     // Success message
        "Some tests failed",    // Failure message
    ),
    (
        "Validating Go code formatting",   // Description
        "gofmt -l .",                      // Command to check if the Go code is properly formatted
        "Code is correctly formatted",     // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Go project documentation",  // Description
        "go doc ./...",                         // Command to generate documentation for Go project
        "Documentation generated successfully", // Success message
        "Failed to generate documentation",     // Failure message
    ),
    (
        "Checking for outdated Go dependencies", // Description
        "go list -m -u all",                     // Command to check for outdated dependencies
        "No outdated dependencies",              // Success message
        "Outdated dependencies detected",        // Failure message
    ),
    (
        "Linting the Go source code",   // Description
        "golangci-lint run", // Command to run linter for Go code (requires golangci-lint)
        "Code linting passed", // Success message
        "Code linting issues detected", // Failure message
    ),
];

#[doc = "All checkup tasks to execute for Python with success and failure messages"]
pub const PYTHON_TASK: [(&str, &str, &str, &str); 9] = [
    (
        "Validating the Python project structure", // Description
        "pip check", // Command to validate dependencies and project structure
        "Project structure and dependencies are valid", // Success message
        "Invalid project structure or dependency issues found", // Failure message
    ),
    (
        "Checking Python project dependencies",       // Description
        "pip freeze > requirements.txt && pip check", // Command to check and freeze dependencies
        "Dependencies are correctly installed",       // Success message
        "Dependency issues detected",                 // Failure message
    ),
    (
        "Checking for Python security vulnerabilities", // Description
        "bandit -r .", // Command to scan for security issues (requires bandit)
        "No security vulnerabilities found", // Success message
        "Security vulnerabilities detected", // Failure message
    ),
    (
        "Running all Python tests", // Description
        "pytest",                   // Command to run Python tests using pytest
        "All tests passed",         // Success message
        "Some tests failed",        // Failure message
    ),
    (
        "Validating Python code formatting", // Description
        "black --check .", // Command to check Python code formatting (requires black)
        "Code is correctly formatted", // Success message
        "Code formatting issues detected", // Failure message
    ),
    (
        "Generating Python project documentation", // Description
        "sphinx-build -b html docs/ build/", // Command to generate documentation (requires Sphinx)
        "Documentation generated successfully", // Success message
        "Failed to generate documentation",  // Failure message
    ),
    (
        "Checking for outdated Python dependencies", // Description
        "pip list --outdated",                       // Command to check for outdated dependencies
        "No outdated dependencies",                  // Success message
        "Outdated dependencies detected",            // Failure message
    ),
    (
        "Linting the Python source code", // Description
        "flake8 .",                       // Command to lint Python code (requires flake8)
        "Code linting passed",            // Success message
        "Code linting issues detected",   // Failure message
    ),
    (
        "Type checking the Python code", // Description
        "mypy .",                        // Command to perform static type checking (requires mypy)
        "No type errors found",          // Success message
        "Type errors detected",          // Failure message
    ),
];
