# Godot 4.4 + Rust GDExtension Template

A minimal, ready-to-use template for Godot 4.4 projects using Rust with the `gdext` bindings.
## Prerequisites
*   **Rust:** Latest stable version (install via [rustup](https://rustup.rs/)).
*   **Godot Engine:** Version 4.4.x or later.
 GDExtension.
*   **Important:** Ensure your development environment is correctly configured by following the **[Official Godot Rust Setup Guide](https://godot-rust.github.io/book/intro/setup.html)**. This template assumes a working `gdext` build environment.
## Quick Start
1.  **Get the Template:**
    *   Click the "Use this template" button on GitHub.
    *   Or, clone the repository:
        ```bash
        git clone https://github.com/uniquadev/godot44-rust.git
        cd godot44-rust
        ```
2.  **Build the Rust Library:**
    ```bash
    cargo build
    ```
This compiles your Rust code (e.g., `rust/src/lib.rs`) into a dynamic library found in `target/debug/`.


3.  **Open in Godot:**
    *   Launch the Godot Engine (4.4+).
    *   Import the `project.godot` file.
    *   Run the main scene (e.g., `main.tscn`) to see the example in action.


For comprehensive `gdext` usage, refer to the [godot-rust book]
