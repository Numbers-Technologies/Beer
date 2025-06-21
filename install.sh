#!/bin/bash
set -e

REPO_URL="https://github.com/Numbers-Technologies/beer"
INSTALL_DIR="/opt/beerpm"
BIN_DIR="/usr/local/bin"

# 0. Check for Rust and Cargo
if ! command -v cargo >/dev/null 2>&1 || ! command -v rustc >/dev/null 2>&1; then
    echo "Rust and Cargo not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
else
    echo "Rust and Cargo found."
fi

# 1. Clone the repo if not present
if [ ! -d "$INSTALL_DIR/beer-src" ]; then
    echo "Cloning BeerPM source..."
    git clone "$REPO_URL" "$INSTALL_DIR/beer-src"
else
    echo "BeerPM source already exists, pulling latest..."
    cd "$INSTALL_DIR/beer-src"
    git pull
fi

# 2. Build the binary
cd "$INSTALL_DIR/beer-src/Beer"
echo "Building BeerPM..."
cargo build --release

# 3. Copy the binary to /usr/local/bin
sudo cp target/release/beer "$BIN_DIR/beer"
sudo chmod +x "$BIN_DIR/beer"
echo "BeerPM installed to $BIN_DIR/beer"

# 4. Set up BeerPM data folders
sudo mkdir -p /opt/beerpm
sudo mkdir -p /opt/beerpm/Packages
sudo mkdir -p /opt/beerpm/Binaries
sudo mkdir -p /opt/beerpm/Formulaes
sudo touch /opt/beerpm/beer.config.toml

# 5. Write info.toml
sudo bash -c '
echo "[info]" > /opt/beerpm/info.toml
installed_count=$(ls /opt/beerpm/Formulaes | wc -l)
disk_usage=$(du -sh /opt/beerpm/Packages | awk "{print \$1}")
echo "installed_count = \$installed_count" >> /opt/beerpm/info.toml
echo "packages_disk_usage = '\$disk_usage'" >> /opt/beerpm/info.toml
'

# 6. Add to PATH if not present
if ! echo "$PATH" | grep -q "/usr/local/bin"; then
    echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
    echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc
    export PATH="/usr/local/bin:$PATH"
    echo "Added /usr/local/bin to PATH. Please restart your shell."
fi

echo "\n🍺 BeerPM installation complete! Run 'beer --help' to get started."


