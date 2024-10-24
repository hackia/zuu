pub mod ask;
pub mod output;
pub mod runner;
pub mod support;

#[doc = "All checkup to execute for Rust"]
pub const RUST_TASK: [&str; 9] = [
    "cargo verify-project", // Verify the project structure and configuration
    "cargo deny check",     // Check for dependency license and security issues
    "cargo check",          // Check for compilation errors without producing binaries
    "cargo audit",          // Audit for known vulnerabilities in dependencies
    "cargo test",           // Run all tests
    "cargo fmt --check",    // Check code formatting
    "cargo doc --no-deps",  // Generate documentation without dependencies
    "cargo outdated",       // Check for outdated dependencies
    "cargo clippy -- -D clippy::all", // Lint the code with Clippy, treating all warnings as errors
];

#[doc = "All checkup to execute for JavaScript"]
pub const JS_TASK: [&str; 9] = [
    "npm install",          // Install dependencies and verify package structure
    "npm audit",            // Audit for vulnerabilities in dependencies
    "npm run lint",         // Run ESLint to check for linting errors
    "npm test",             // Run all tests using a testing framework (e.g., Jest, Mocha)
    "npm run format:check", // Check code formatting (e.g., with Prettier)
    "npm run build",        // Build the project and verify if there are build errors
    "npm run doc",          // Generate documentation (e.g., with JSDoc)
    "npm outdated",         // Check for outdated dependencies
    "npm run lint:fix",     // Run ESLint and automatically fix issues
];

#[doc = "All checkup to execute for php"]
pub const PHP_TASK: [&str; 9] = [
    "composer validate",
    "composer licenses",
    "composer check-platform-reqs",
    "composer audit",
    "composer run-script test",
    "composer run-script fmt",
    "composer run-script doc",
    "composer outdated",
    "composer run-script lint",
];

#[doc = "All checkup to execute for D"]
pub const D_TASK: [&str; 9] = [
    "dub describe",             // Vérifie la configuration du projet
    "dub lint --compiler=dmd",  // Vérifie les licences et l'intégrité
    "dub build --compiler=dmd", // Compile le projet et vérifie les erreurs
    "dub test --compiler=dmd",  // Exécute tous les tests
    "dub format",               // Vérifie le formatage du code
    "dub generate-doc",         // Génère la documentation
    "dub upgrade",              // Vérifie et met à jour les dépendances
    "dub lint",                 // Linting du code
    "snyk test",                // Audite les vulnérabilités avec Snyk (si installé)
];

#[doc = "All checkup to execute for Go"]
pub const GO_TASK: [&str; 9] = [
    "go mod tidy",       // Nettoyer le fichier go.mod (équivalent à composer validate)
    "go mod verify",     // Vérifier les dépendances (équivalent à composer licenses)
    "go mod vendor",     // Mettre à jour les dépendances dans le répertoire vendor
    "go audit",          // Utiliser un outil comme govulncheck pour l'audit de sécurité
    "go test ./...",     // Exécuter tous les tests (équivalent à composer run-script test)
    "go fmt ./...",      // Formater tout le code Go (équivalent à composer run-script fmt)
    "go doc ./...",      // Générer la documentation (équivalent à composer run-script doc)
    "go list -m -u all", // Vérifier les mises à jour des dépendances (équivalent à composer outdated)
    "golangci-lint run ./...", // Linting du code avec golangci-lint (équivalent à composer run-script lint)
];

#[doc = "All checkup to execute for Python"]
pub const PYTHON_TASK: [&str; 9] = [
    "pip check",    // Vérifier les dépendances installées (équivalent à composer validate)
    "pip-licenses", // Lister les licences des dépendances installées (équivalent à composer licenses)
    "pip install -r requirements.txt", // Vérifier les dépendances du projet (équivalent à composer check-platform-reqs)
    "pip-audit", // Auditer les vulnérabilités dans les dépendances (équivalent à composer audit)
    "pytest",    // Exécuter tous les tests (équivalent à composer run-script test)
    "black --check .", // Vérifier le formatage du code avec Black (équivalent à composer run-script fmt)
    "pdoc --html .",   // Générer la documentation HTML (équivalent à composer run-script doc)
    "pip list --outdated", // Lister les dépendances obsolètes (équivalent à composer outdated)
    "pylint .",        // Exécuter le linting avec Pylint (équivalent à composer run-script lint)
];
