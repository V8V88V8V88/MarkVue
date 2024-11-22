# MarkVue

MarkVue is a sleek, GTK-based Markdown viewer that allows you to write Markdown and see the rendered output in real-time.

## Features

- Real-time Markdown rendering
- Syntax highlighting for Markdown input
- Split-pane interface for easy editing and previewing
- Built with GTK4 for a modern, native look and feel

## Installation

To install MarkVue, you'll need Rust and Cargo installed on your system. Then, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/v8v88v8v88/markvue.git
   cd markvue
   ```

2. Build and run the application:
   ```bash
   cargo run
   ```

3. Open the `MarkVue` project:
   - Open the project directory where you cloned the repository.
   - Navigate to `src/main.rs` to modify the application code or add new features.

## Dependencies

MarkVue relies on the following Rust crates:

- gtk4 (0.9)
- gio (0.19)
- glib (0.19)
- sourceview5 (0.9)
- pulldown-cmark (0.10)

Make sure you have the necessary GTK4 development libraries installed on your system.

## Usage

1. Launch MarkVue
2. Start typing Markdown in the left pane
3. See the rendered output in real-time on the right pane
4. Use the hamburger menu to access additional options

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

[v8v88v8v88](https://github.com/v8v88v8v88)

## Acknowledgments

- The GTK Project for providing the GUI toolkit
- The Rust community for their excellent crates and support
