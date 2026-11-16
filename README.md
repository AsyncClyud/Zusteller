# Zusteller – A Lightweight Rust API Client

## Description

**Zusteller** is a lightweight desktop HTTP client written in Rust. It provides a simple and responsive interface for sending HTTP requests, inspecting server responses, and testing APIs without the overhead of larger API development tools.

Built with Rust, `egui`, and `reqwest`, Zusteller focuses on low resource consumption, fast startup, and a straightforward user experience.

### Highlights

- **Fast startup:** approximately 0.4 seconds on the tested system.
- **Small distribution size:** AppImage approximately 5 MB.
- **Low memory footprint:** approximately 49 MiB PSS / 62 MiB RSS when idle on the tested system.
- **Native desktop UI:** built with `egui` using OpenGL rendering.

## Features

- HTTP methods: `GET`, `POST`, `PUT`, `PATCH`, `DELETE`.
- Request body support for raw text and JSON.
- Response viewing with scrolling.
- Custom HTTP headers *(in development)*.
- Customizable `User-Agent`.

### Planned Features

- Multipart form data and file uploads.
- Customizable `User-Agent`.
- Improved HTTP status-code information.
- Request history and collections.

## Technical Stack

- **Language:** Rust.
- **HTTP Client:** `reqwest` (asynchronous HTTP requests).
- **Async Runtime:** `tokio`.
- **GUI Framework:** `eframe` + `egui`.
- **Rendering:** `egui_glow` (native OpenGL).
- **Distribution:** AppImage (Linux x86_64).

## Requirements

### Running

- 64-bit Linux distribution.
- Compatible graphics driver with OpenGL support.

### Building from Source

- Rust toolchain and Cargo.
- `linuxdeploy` for AppImage packaging.

## Installation

1. Download the latest AppImage from the [Releases](https://github.com/AsyncClyud/Zusteller/releases) page.
2. Make it executable:

   ```bash
   chmod +x Zusteller-x86_64.AppImage
   ```

3. Launch:

   ```bash
   ./Zusteller-x86_64.AppImage
   ```

## Project Structure

```text
Zusteller/
├── src/
│   ├── api/
│   ├── ui/
│   └── main.rs
├── assets/
├── packaging/
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Roadmap

| Feature | Status |
|---|---|
| Basic HTTP methods | Implemented |
| Request body | Implemented |
| Response copying | Implemented |
| Custom HTTP headers | In Development |
| Customizable User-Agent | Implemented |
| PATCH support | Implemented |
| File uploads | Planned |
| Request history | Planned |
| HTTP status-code documentation | Planned |
| Windows/macOS support | Planned |

## Contributing

Contributions are welcome!

1. Fork the repository.
2. Create a feature branch:

   ```bash
   git checkout -b feature/your-feature
   ```

3. Commit your changes:

   ```bash
   git commit -m "Add feature X"
   ```

4. Open a Pull Request.

Before submitting a PR:

- Run `cargo test`.
- Update relevant documentation.
- Follow the existing project structure and coding style.

## License

This project is licensed under the **Apache License 2.0**. See [LICENSE](LICENSE) for details.
