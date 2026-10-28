#!/bin/bash

echo "🔧 Setting up remnant-core development environment..."

OS="$(uname)"

if [[ "$OS" == "Darwin" ]]; then
    echo "🍎 macOS detected"

    # Check Homebrew
    if ! command -v brew &> /dev/null; then
        echo "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi

elif [[ "$OS" == "Linux" ]]; then
    echo "🐧 Linux detected"
fi

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "⚠️ Docker is required for local development."
    echo "Install Docker Desktop from https://www.docker.com/products/docker-desktop"
    echo "Then re-run this script."
    exit 1
fi

echo "Starting local services with Docker..."
docker compose up -d redis
if [ $? -ne 0 ]; then
    echo "❌ Failed to start Docker services. Make sure Docker Desktop is running."
    exit 1
fi
echo "✅ Redis running on localhost:6379"

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

echo "Updating Rust to stable..."
rustup update stable
rustup default stable

# Install required Rust components
echo "Installing Rust components..."
rustup component add rustfmt clippy

# Install cargo tools
echo "Installing cargo tools..."
cargo install cargo-audit          # Security vulnerability scanner
cargo install sea-orm-cli          # Database migration tool
cargo install cargo-watch          # Auto-reload on file changes during development
cargo install git-cliff            # Changelog generator
cargo install typos-cli            # Type scanner

# Copy env file if not exists
if [ ! -f .env ]; then
    echo "Creating .env from .env.example..."
    cp .env.example .env
    echo "⚠️  Fill in your credentials in .env before running the server"
fi

echo "✅ remnant-core setup complete"
echo ""
echo "Next steps:"
echo "  1. Fill in .env with your credentials"
echo "  2. Run 'cargo build' to compile"
echo "  3. Run 'sea-orm-cli migrate up' to run database migrations"
echo "  4. Run 'cargo run' to start the server"
echo "  5. Run 'cargo watch -x run' for auto-reload during development"
