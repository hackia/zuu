# Zuu : Un outil puissant et flexible pour la vérification du code Rust (et au-delà)

<img src="logo.png" alt="zuu" width ="350" height="350" align="right"/>

Zuu est un outil en ligne de commande conçu pour simplifier et améliorer la vérification du code pour les projets Rust. Il utilise un fichier de configuration `zuu.toml` à la racine du projet, permettant aux développeurs de définir une série de vérifications et d'actions à exécuter avant, pendant et après les commandes `cargo`.

**Fonctionnalités principales :**

* **Hooks personnalisables :** Zuu prend en charge les hooks `before-cargo`, `cargo` et `after-cargo`, permettant aux développeurs d'exécuter des commandes shell à différentes étapes du processus de vérification.
* **Configuration flexible :** Le fichier `zuu.toml` offre un moyen centralisé et adaptable de gérer les vérifications de code, y compris les linters, les formateurs, les frameworks de test et tout autre outil de ligne de commande.
* **Prise en charge intégrée de `clippy` :** Zuu inclut une prise en charge intégrée de `clippy`, le linter populaire de Rust, avec des options pour personnaliser les lints et les niveaux de sévérité.
* **Extensible avec des scripts shell :** Les hooks peuvent exécuter n'importe quelle commande shell, permettant des scripts complexes, une logique conditionnelle, la manipulation de fichiers et l'intégration avec des outils externes.
* **Flux de travail automatisés :** Zuu peut être intégré de manière transparente dans les pipelines CI/CD et les hooks Git, permettant une vérification automatisée du code à chaque commit ou pull request.
* **Génération de badges :** Zuu peut générer des badges indiquant l'état du processus de vérification du code, fournissant une représentation visuelle de la qualité du code.

**Avantages :**

* **Amélioration de la qualité du code :** Zuu aide à appliquer les normes de codage, à identifier les problèmes potentiels et à garantir la cohérence du code.
* **Efficacité accrue :** L'automatisation de la vérification du code permet de gagner du temps et de réduire les risques d'erreur humaine.
* **Collaboration améliorée :** Zuu favorise la collaboration en fournissant un cadre partagé pour la qualité du code.
* **Plus grande flexibilité :** La possibilité d'exécuter n'importe quelle commande shell offre un haut degré de personnalisation et d'extensibilité.

**Projets futurs :**

* **Prise en charge multilingue :** Zuu prévoit d'étendre sa prise en charge pour inclure d'autres langages comme D et Go, ce qui en fera un outil universel de vérification de code.
* **Système de plugins :** Un système de plugins est envisagé pour améliorer encore l'extensibilité et la participation de la communauté.

**Exemple de configuration `zuu.toml` :**

```toml
before-cargo = ["cargo fmt"]
cargo = [
    "cargo verify-project",
    "cargo check --all-targets --profile=test",
    "cargo deny check",
    "cargo audit",
    "cargo test -j 4 --no-fail-fast -- --show-output",
    "cargo fmt --check",
    "cargo clippy -- -D clippy::pedantic -W clippy::nursery -D warnings -D clippy::all",
    "cargo outdated",
]
after-cargo = []

[badge]
success = ["curl https://img.shields.io/badge/zuu-passing-darkgreen -o zuu.svg"]
failure = ["curl https://img.shields.io/badge/zuu-failure-red -o zuu.svg"]
```

**Pour commencer :**

Actuellement, la meilleure façon de commencer avec Zuu est de l'installer directement à partir de son référentiel GitHub et de se référer à sa documentation. Comme il est encore en développement, les instructions d'installation et d'utilisation peuvent évoluer.

Zuu est un outil puissant et polyvalent qui permet aux développeurs Rust de prendre le contrôle de la qualité de leur code. Sa flexibilité, son extensibilité et sa focalisation sur l'automatisation en font un atout précieux pour tout projet Rust.

![demo](https://raw.githubusercontent.com/otechdo/zuu/refs/heads/main/zuu.23.gif)

## Installation

### Non graphique

```bash
cargo install zuu --no-default-features --features cli 
```

### Pour l'insatllation graphi

```bash
cargo install zuu broot
```

## Usage

```bash
zuu
```

## Github workflow

```yaml
name: zuu
on:
  push:
    branches: [ "master" , "develop" , "main" ]
  pull_request:
    branches: [ "master" , "develop" , "main"  ]
env:
  CARGO_TERM_COLOR: always
  TERM: xterm-256color
jobs:
  zuu:
    strategy:
      matrix:
        os: [ ubuntu-latest, macos-latest ]
    runs-on: ${{ matrix.os }}
    steps:
    - uses: actions/checkout@v3
    - name: deps
      run:  cargo install cargo-audit cargo-auditable cargo-deny cargo-outdated
    - name: installation
      run:  cargo install zuu --no-default-features --features cli
    - name: zuu
      run: git checkout "${GITHUB_REF##*/}" && zuu
```
