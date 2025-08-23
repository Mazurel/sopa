# SOPa

**SOP** *Aphia* is an open-source web application designed to help people find support centers and social assistance locations.

## 🌟 Features

- **Location Finder**: Search and discover support centers based on your specific needs
- **Tag-based Filtering**: Filter locations by various categories
- **Location Management**: Add, edit, and manage support center information
- **Dark/Light Mode**: Automatic color scheme detection and switching
- **Data Import/Export**: Save and share location databases as `.bson` files

## 🏗️ Architecture

This project is built as a Rust workspace with two main components:

- **`frontend/`** - Web application built with [Yew](https://yew.rs/) framework
- **`libsopa/`** - Core library containing shared data structures and business logic
- **`locales/`** - Localization files for internationalization
- **`.github/`** - CI configuration files for continuous integration and deployment

### Database Architecture

The application uses a sophisticated multi-layer database system designed for web-based operation:

#### Storage Layers

1. **Browser Persistence (IndexedDB)**
   - Primary storage using the browser's IndexedDB API
   - Automatic synchronization between memory and persistent storage

2. **BSON Serialization**
   - Location data serialized to Binary JSON (BSON) format
   - Enables efficient import/export of entire databases
   - Backwards-compatible schema with migration support
   - Default database embedded as binary data in the application

3. **In-Memory Operations**
   - HashMap-based storage (`HashMap<Uuid, Location>`) for fast lookups
   - UUID-based unique identification for all locations
   - Tag-based filtering and preference-based sorting algorithms

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Trunk](https://trunkrs.dev/) for building the frontend

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Development

1. Clone the repository:
```bash
git clone https://github.com/your-username/sopa.git
cd sopa
```

2. Start the development server:
```bash
cd frontend
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve
```

3. Open your browser and navigate to `http://localhost:8080`

### Building for Production

```bash
cd frontend
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk build --release
```

The built application will be available in the `frontend/dist/` directory.

## 🌍 Internationalization

The application supports multiple languages through the `rust-i18n` crate:
- Polish (pl)
- English (en)

Translations are stored in the `locales/` directory as YAML files.

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## ⚠️ Development Status

**This is alpha software currently in active development.** Features may be incomplete, unstable, or subject to change. Use for testing and feedback purposes only.

## 📄 License

This project is licensed under the GNU General Public License v2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Yew](https://yew.rs/) - A modern Rust framework for creating multi-threaded frontend web apps with WebAssembly
- UI components powered by [Bulma](https://bulma.io/) CSS framework
