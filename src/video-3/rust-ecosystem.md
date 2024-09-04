### Rust Ecosystem 

Tools and utilities to perform many things around rust programming language. 

On a higher level `Cargo` seems to be a swiss-knife for Rust. As a person coming from Python, I can vouch how much of a need is that. Here is a basic comparison between Python and Rust tooling. Truely :octocat: :exploding_head:!!


|Purpose|Python|Rust|
|---|---|---|
| Linter | pylint, flake8,... | cargo clippy|
| LSP | python-language-server, ruff-lsp | rust-analyzer|
|formatting |ruff, black | cargo fmt |
|building binary |  setuptools, py2exe| cargo build|
| test | unittest, pytest | cargo test|
| dev environment | virtualenv, pipx, pip-tools, uv, conda, pdm and what not! | cargo new, cargo update|
|documentation |sphinx, mkdocs | cargo doc|
|benchmarking |cProfile | cargo bench, criterion*|

Among all the aboce, only `criterion` is an external tool from `cargo`. Like, WTF Cargo?!?!? Why no such tool in Python? Currently astral want to build cargo like tool for Python, but already built two different tools. :thinking: Let's see the journey! 