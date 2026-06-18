# Tincta V2

[![CI](https://github.com/sindus/tincta-V2/actions/workflows/ci.yml/badge.svg)](https://github.com/sindus/tincta-V2/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A fast, stable, cross-platform text editor for **macOS** and **Linux**, inspired by [Tincta for macOS](https://github.com/CodingFriends/Tincta). Built in Rust with [iced](https://github.com/iced-rs/iced).

**[Website & Downloads](https://sindus.github.io/tincta-V2)**

---

## Features

- **Syntax highlighting** for 60+ languages (TextMate grammars)
- **File sidebar** — open and switch between multiple files
- **Search & Replace** with regex support
- **Dark / Light theme**
- **Internationalisation** — English and French
- Configurable: font size, tab width, word wrap, auto-indent, bracket/quote completion
- Line editing: duplicate, move, comment/uncomment, indent/dedent
- Code formatting via external tools (prettier, rustfmt, …)
- Native binaries — macOS (Apple Silicon) and Linux (x86_64)

---

## Installation

### One-liner (Linux & macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/sindus/tincta-V2/main/install.sh | bash
```

Detects your OS automatically, downloads the latest release, and installs it.

---

### macOS — Homebrew

```bash
brew tap sindus/tincta
brew install tincta-v2
```

**Uninstall:**
```bash
brew uninstall tincta-v2
```

---

### Ubuntu / Debian — .deb

Download and install the latest `.deb` package:

```bash
curl -fsSL https://api.github.com/repos/sindus/tincta-V2/releases/latest \
  | grep '"browser_download_url"' | grep '\.deb' \
  | cut -d '"' -f 4 | xargs wget -q -O tincta.deb
sudo dpkg -i tincta.deb && rm tincta.deb
```

Or grab the file directly from the [Releases page](https://github.com/sindus/tincta-V2/releases/latest).

**Uninstall:**
```bash
sudo apt remove tincta
# or
sudo dpkg -r tincta
```

---

### Linux — tar.gz (any distro)

```bash
VERSION=$(curl -fsSL https://api.github.com/repos/sindus/tincta-V2/releases/latest | grep '"tag_name"' | head -1 | sed 's/.*"\(.*\)".*/\1/')
curl -fsSL "https://github.com/sindus/tincta-V2/releases/download/${VERSION}/tincta-${VERSION}-x86_64-linux.tar.gz" | tar xz
sudo mv tincta /usr/local/bin/
```

**Uninstall:**
```bash
sudo rm /usr/local/bin/tincta
```

---

### Build from source

```bash
# Prerequisites (Ubuntu/Debian)
sudo apt-get install libgtk-3-dev libxkbcommon-dev

# Build & run
cargo build --release
./target/release/tincta
```

---

## Usage

```bash
tincta                  # open with last session
tincta path/to/file     # open a specific file
```

---

## Development

```bash
cargo run           # run in dev mode
cargo test          # run tests
cargo clippy        # lint
cargo fmt           # format source
```

---

## License

MIT — see [LICENSE](LICENSE).

Inspired by [Tincta](https://github.com/CodingFriends/Tincta) by CodingFriends.
