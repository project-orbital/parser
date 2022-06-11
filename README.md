# Parser
This repository contains the document parser of DollarPlanner. The parser is integrated with the backend as a submodule without a standalone executable.

`master` is the stable branch, and should be able to be integrated with the corresponding `master`
branch of the backend.

`dev` is the main development branch, which may be unstable and buggy.
Integration with the backend is not guaranteed to work.

## Developer Setup

### System requirements
1. [Rust](https://www.rust-lang.org/tools/install) 1.61.0 or higher
2. [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) 0.10.2 or higher

### Setting up your local environment
1. Clone the repository to your local machine.

    ```
    cd <clone location>
    git clone https://github.com/project-orbital/parser
    ```

2. Install all the dependencies.

    ```
    cd parser
    cargo build
    ```

3. Run the unit tests.

    ```
    cargo test
    ```
