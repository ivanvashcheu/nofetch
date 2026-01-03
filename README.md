# nofetch
nofetch is a CLI application that displays current system information in a single line. it can be used as a greeting when opening a new terminal instance.

# nofetch

a lightweight, blazing-fast command-line application written in rust that displays essential system information in a single, compact line. perfect for a minimal, informative greeting when opening a new terminal instance.

## features

*   **single-line output:** all your system's vital stats presented concisely in one line, saving precious screen space.
*   **linux-first:** designed specifically for linux environments, fetching information directly from the system.
*   **blazing fast:** written in rust for near-instantaneous execution. no noticeable lag on shell startup.
*   **minimal & unobtrusive:** does one job and does it well. no bloat, no distractions.

## information displayed

a typical `nofetch` output looks like this (exact format may vary):
`👋 ivanv | 🐧 6.17.12-300.fc43.x86_64 | ⏱️  03h:29m | 🐚 fish | 💾 5.6/31.3 GiB | 💿 50.8/463.2 GB`

the application gathers and displays:
*   **user**
*   **kernel** version
*   **system uptime**
*   **current shell**
*   **memory** usage (used/total)
*   **root disk** usage (used/total)

## installation

### prerequisites
*   a linux system.
*   rust toolchain (install via [rustup.rs](https://rustup.rs/)).
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

### from source (recommended)
1.  clone the repository:
    ```bash
    git clone https://github.com/ivanvashcheu/nofetch.git
    cd nofetch
    ```
2.  build and install with cargo:
    ```bash
    cargo install --path .
    ```
    this will compile the binary and place it in `~/.cargo/bin/` (ensure this directory is in your `path`).

### using cargo (from crates.io - if it will be published)
*(future release)*
```bash
cargo install nofetch
```

## 🖥️ usage

run `nofetch` directly from your terminal:
```bash
$ nofetch
👋 ivanv | 🐧 6.17.12-300.fc43.x86_64 | ⏱️  03h:29m | 🐚 fish | 💾 5.6/31.3 GiB | 💿 50.8/463.2 GB
```

### set as terminal greeting

add the `nofetch` command to your shell's startup file to see it every time you open a new terminal.

**for bash** (`~/.bashrc` or `~/.bash_profile`):
```bash
echo "nofetch" >> ~/.bashrc
```

**for zsh** (`~/.zshrc`):
```bash
echo "nofetch" >> ~/.zshrc
```

**for fish** (`~/.config/fish/config.fish`):
```bash
echo "nofetch" >> ~/.config/fish/config.fish
```

after editing, reload your shell configuration or open a new terminal window.

## building from source

to build for development or contribution:
```bash
# clone the repo
git clone https://github.com/ivanvashcheu/nofetch.git
cd nofetch

# build in debug mode
cargo build

# run the built binary
./target/debug/nofetch

# build an optimized release binary
cargo build --release
# the binary will be at ./target/release/nofetch
```

## 🤝 contributing

contributions are welcome! whether it's adding support for more system metrics, improving performance, fixing bugs, or enhancing formatting.

1.  fork the repository.
2.  create a feature branch (`git checkout -b feature/amazing-feature`).
3.  commit your changes (`git commit -m 'add some amazing feature'`).
4.  push to the branch (`git push origin feature/amazing-feature`).
5.  open a pull request.

please ensure your code follows the existing style and includes appropriate documentation.

## ⚖️ license

this project is licensed under the GNU GPLv3 license - see the [license](https://www.gnu.org/licenses/gpl-3.0.html) for details.

## acknowledgments

*   inspired by system information tools like `screenfetch`, `neofetch`, and `fastfetch`.

## 📧 contact

Ivan Vashcheu - ivashcheu@proton.me

project link: [https://github.com/ivanvashcheu/nofetch](https://github.com/ivanvashcheu/nofetch)
