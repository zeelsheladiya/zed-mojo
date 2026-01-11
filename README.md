# Mojo for Zed

[Mojo](https://docs.modular.com/mojo/manual) language support for the [Zed editor](https://zed.dev).

## Features

- **Language Server Protocol**: Deep integration with `mojo-lsp` for a full IDE experience.
    - **Smart Detection**: Automatically finds the language server in your `VIRTUAL_ENV`, project `.venv`, `~/.modular` installation, or system `PATH`.
    - **Robust Environment**: Inherits your full shell environment (`PATH`, `PYTHONPATH`) and cleanly injects `MODULAR_HOME`, ensuring correct execution of subprocesses and Python interop.
    - **Features**: Diagnostics, Hover, Go to Definition, Autocompletion.
- **Robust Syntax Highlighting**: Uses the official Tree-sitter grammar for accurate highlighting of all Mojo features (`struct`, `fn`, `borrowed`, etc.).

## Installation

1. Open Zed.
2. Open the Command Palette (`Cmd+Shift+P`).
3. Search for "Extensions: Install Extension".
4. Select "Mojo".

## Configuration

The extension requires no manual configuration for most users. It automatically searches for the Mojo Language Server in the following order:

1. Active `VIRTUAL_ENV` environment variable.
2. Local project virtual environments (`.venv` or `venv` folders).
3. System `PATH` (`mojo-lsp-server`, `mojo-lsp`, or `mojo`).
4. Standard Modular installation directory (`~/.modular/...`).

### Syntax Highlighting

The extension uses the official [lsh/tree-sitter-mojo](https://github.com/lsh/tree-sitter-mojo) grammar for high-quality syntax highlighting.

## Requirements

- **Mojo SDK**: You must have the Mojo SDK installed. The extension does not bundle the compiler or language server.
  - [Install Mojo](https://docs.modular.com/mojo/manual/get-started)

## Troubleshooting

- **Language Server not starting**:
  - The extension looks for `mojo-lsp-server`, `mojo-lsp`, or `mojo`.
  - If the server is not found, the extension will display a **verbose error message** showing exactly which paths were checked and what your environment (PATH, CWD) looks like. Please use this information to verify your setup.
  - Ensure you have run `mojo --version` in your terminal to verify your installation.

## License

MIT
