<img width="340" height="340" alt="icon-1" src="https://github.com/user-attachments/assets/dacf4768-fc8e-4c37-8f55-ca694a12b420" />


**Zusteller – A Lightweight Rust API Client**

---

### **Description**
**Zusteller** is a high-performance, resource-efficient HTTP request client written in Rust. Designed for developers and testers who need a fast, lightweight tool for API interactions without unnecessary overhead.

Key advantages:
- **Instant startup** (~0.4 seconds).
- **Minimal footprint** (AppImage ~5 MB).
- **Low resource consumption** (15 MB GPU memory, 50 MB RAM idle).
- **Simple UI** built with `egui` for intuitive usability.

---

### **Features**
- Supports core HTTP methods: `GET`, `POST`, `PUT`, `PATCH`, `DELETE`.
- Request body support for JSON, forms, and file uploads.
- One-click response copying.
- **Upcoming improvements**:
  - Custom HTTP headers.
  - Customizable `User-Agent`.
  - HTTP error documentation.

---

### **Technical Stack**
- **Backend**: `reqwest` (async HTTP requests) + `tokio` (runtime).
- **Frontend**: `eframe` (desktop app framework) + `egui` (UI library).
- **Rendering**: `glow` (OpenGL via WebAssembly, adapted for desktop).
- **Distribution**: AppImage (Linux).

---

### **Requirements**
- Rust (1.99.1 or later).
- For AppImage builds: `linuxdeploy` and dependencies.

---

### **Installation**
1. **Download the AppImage** (Linux) from the [Releases](https://github.com/AsyncClyud/Zusteller/releases) section.
2. Make it executable:
   ```bash
   chmod +x Zusteller-x86_64.AppImage
   ```
3. Run:
   ```bash
   ./Zusteller-x86-64.AppImage
   ```

---

### **Project Structure**
```
Zusteller/
├── src/
│   ├── api/      # HTTP request logic
│   ├── ui/       # egui-based interface
│   └── main.rs   # Entry point
├── Cargo.toml    # Dependencies and config
└── README.md     # Documentation
```

---

### **Documentation**
- [API Documentation](docs/api.md) *(in progress)*.
- [Usage Guide](docs/usage.md) *(in progress)*.

---

### **Roadmap**
| Feature               | Status          | Notes                          |
|-----------------------|-----------------|--------------------------------|
| Custom Headers        | In Development  | Planned             |
| User-Agent Customization | In Development | For now we have 'Zusteller/0.0.1' User-Agent            |
| macOS/Windows Support | Planned         | After Linux stability           |
| HTTP Error Docs       | Planned         | Post-core features              |

---

### **Contributing**
1. **Fork** the repository.
2. **Create a branch** (`git checkout -b feature/your-feature`).
3. **Commit changes** (`git commit -m 'Add feature X'`).
4. **Open a Pull Request**.

Before submitting a PR, ensure:
- Code passes `cargo test`.
- Documentation is updated.
- Changes align with project style.

---

### **License**
This project is licensed under **Apache 2.0**. See [LICENSE](LICENSE) for details.

---
**Note**: Rust compiler is required for stable operation. macOS/Windows support will be added later.

---
