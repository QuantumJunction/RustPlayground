# RustPlayground

A simple space for experimenting with Rust snippets, testing features, and trying out ideas. Nothing here is meant to be tidy or final — it’s just a sandbox.

## Features

- Minimal structure
- Quick experiments without heavy setup
- Optional isolated Cargo projects for different tests
- Room for standalone .rs files or small modules

## Getting Started
1. Clone the repo
```bash
git clone <your-repo-url>
cd <repo-name>
```

2. Install Rust (if not already installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Update dependencies (if using Cargo projects)
```bash
cargo update
```

## Structure
```bash
/
├── experiments/        # Small Cargo projects or isolated tests
│   ├── exp1/
│   │   └── src/main.rs
│   └── exp2/
├── scratch/            # Loose .rs files without a full Cargo setup
├── utils/              # Reusable helper modules
├── LICENSE
└── README.md
```

## Usage
Compile and run a single Rust file
```bash
rustc scratch/<file>.rs
./<file>
```

Run a Cargo project
```bash
cargo run --manifest-path experiments/<project>/Cargo.toml
```

Run tests
```bash
cargo test
```

## Guidelines

- Keep experiments separated so they don’t interfere with each other.
-Throw away what’s not useful.
-Anything that might be handy more than once goes into utils/.
