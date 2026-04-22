#!/usr/bin/env sh

set -e

REPO="StanleyMasinde/axum-template"
BIN_NAME="axum_template"
INSTALL_DIR="${AXUM_TEMPLATE_INSTALL:-/usr/local/bin}"
VERSION="${1:-latest}"

detect_platform() {
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    case "$os" in
        linux*) os="linux" ;;
        darwin*) os="darwin" ;;
        mingw*|msys*|cygwin*) os="windows" ;;
        *) echo "Error: Unsupported OS: $os" >&2; exit 1 ;;
    esac

    arch=$(uname -m)
    case "$arch" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        armv7l|armv6l) arch="arm" ;;
        *) echo "Error: Unsupported architecture: $arch" >&2; exit 1 ;;
    esac

    echo "${os}-${arch}"
}

get_release_data() {
    version="$1"

    if [ "$version" = "latest" ]; then
        api_url="https://api.github.com/repos/$REPO/releases/latest"
    else
        api_url="https://api.github.com/repos/$REPO/releases/tags/$version"
    fi

    curl -fsSL "$api_url" || {
        echo "Error: Could not fetch release data" >&2
        exit 1
    }
}

parse_version() {
    json="$1"
    echo "$json" | grep -o '"tag_name"[[:space:]]*:[[:space:]]*"[^"]*"' | head -1 | sed 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/'
}

parse_asset() {
    json="$1"
    filename="$2"

    asset_block=$(echo "$json" | sed -n "/\"name\"[[:space:]]*:[[:space:]]*\"${filename}\"/,/\"browser_download_url\"/p")
    url=$(echo "$asset_block" | grep '"browser_download_url"' | head -1 | sed 's/.*"browser_download_url"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/')
    digest=$(echo "$asset_block" | grep '"digest"' | head -1 | sed 's/.*"sha256:\([^"]*\)".*/\1/')

    if [ -n "$url" ]; then
        echo "${url}|${digest}"
    fi
}

verify_checksum() {
    file="$1"
    expected_sha="$2"

    if [ -z "$expected_sha" ] || [ "$expected_sha" = "null" ]; then
        echo "Warning: No checksum available for this release"
        echo "Skipping verification"
        return 0
    fi

    echo "Verifying checksum..."

    if command -v sha256sum >/dev/null 2>&1; then
        actual_sha=$(sha256sum "$file" | awk '{print $1}')
    elif command -v shasum >/dev/null 2>&1; then
        actual_sha=$(shasum -a 256 "$file" | awk '{print $1}')
    else
        echo "Warning: Neither sha256sum nor shasum found"
        echo "Cannot verify checksum"
        return 0
    fi

    if [ "$actual_sha" = "$expected_sha" ]; then
        echo "Checksum verified: $expected_sha"
        return 0
    fi

    echo "Checksum verification failed" >&2
    echo "Expected: $expected_sha" >&2
    echo "Got:      $actual_sha" >&2
    return 1
}

install_binary() {
    version="$1"
    platform="$2"

    case "$platform" in
        windows-*) archive_ext="zip" ;;
        *) archive_ext="tar.gz" ;;
    esac

    filename="${platform}.${archive_ext}"

    echo "Installing ${BIN_NAME}"
    echo "Fetching release information..."

    release_json=$(get_release_data "$version")

    if [ "$version" = "latest" ]; then
        version=$(parse_version "$release_json")
        if [ -z "$version" ]; then
            echo "Error: Could not parse version from API response" >&2
            exit 1
        fi
    fi

    echo "Version:  $version"
    echo "Platform: $platform"

    asset_info=$(parse_asset "$release_json" "$filename")
    if [ -z "$asset_info" ]; then
        echo "Error: Could not find asset '$filename' in release" >&2
        echo "Available assets:" >&2
        echo "$release_json" | grep -o "\"name\"[[:space:]]*:[[:space:]]*\"[^\"]*\"" | sed 's/.*"name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/  - \1/' >&2
        exit 1
    fi

    download_url=$(echo "$asset_info" | cut -d'|' -f1)
    sha256_digest=$(echo "$asset_info" | cut -d'|' -f2)

    tmp_dir=$(mktemp -d)
    original_dir=$(pwd)
    cd "$tmp_dir"

    echo "Downloading from: $download_url"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL --progress-bar -o "$filename" "$download_url" || {
            cd "$original_dir"
            rm -rf "$tmp_dir"
            echo "Error: Download failed" >&2
            exit 1
        }
    elif command -v wget >/dev/null 2>&1; then
        wget -q --show-progress -O "$filename" "$download_url" || {
            cd "$original_dir"
            rm -rf "$tmp_dir"
            echo "Error: Download failed" >&2
            exit 1
        }
    else
        cd "$original_dir"
        rm -rf "$tmp_dir"
        echo "Error: Neither curl nor wget found" >&2
        exit 1
    fi

    verify_checksum "$filename" "$sha256_digest" || {
        cd "$original_dir"
        rm -rf "$tmp_dir"
        exit 1
    }

    echo "Extracting..."
    case "$archive_ext" in
        tar.gz)
            tar -xzf "$filename" || {
                cd "$original_dir"
                rm -rf "$tmp_dir"
                echo "Error: Extraction failed" >&2
                exit 1
            }
            extracted_bin="$BIN_NAME"
            ;;
        zip)
            if command -v unzip >/dev/null 2>&1; then
                unzip -q "$filename" || {
                    cd "$original_dir"
                    rm -rf "$tmp_dir"
                    echo "Error: Extraction failed" >&2
                    exit 1
                }
            else
                cd "$original_dir"
                rm -rf "$tmp_dir"
                echo "Error: unzip not found" >&2
                exit 1
            fi
            extracted_bin="${BIN_NAME}.exe"
            ;;
    esac

    if [ ! -f "$extracted_bin" ]; then
        cd "$original_dir"
        rm -rf "$tmp_dir"
        echo "Error: Binary not found after extraction" >&2
        exit 1
    fi

    chmod +x "$extracted_bin" 2>/dev/null || true

    echo "Installing to $INSTALL_DIR..."
    if [ ! -d "$INSTALL_DIR" ]; then
        if mkdir -p "$INSTALL_DIR" 2>/dev/null; then
            :
        else
            sudo mkdir -p "$INSTALL_DIR"
        fi
    fi

    if [ -w "$INSTALL_DIR" ]; then
        install -m 755 "$extracted_bin" "$INSTALL_DIR/$BIN_NAME"
    else
        sudo install -m 755 "$extracted_bin" "$INSTALL_DIR/$BIN_NAME"
    fi

    cd "$original_dir"
    rm -rf "$tmp_dir"

    echo "Installed to $INSTALL_DIR/$BIN_NAME"
    echo "Run '$BIN_NAME --help' to get started"
}

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    cat <<EOF
${BIN_NAME} installer

Usage:
  curl -fsSL https://raw.githubusercontent.com/$REPO/main/install.sh | sh

Or with a specific version:
  curl -fsSL https://raw.githubusercontent.com/$REPO/main/install.sh | sh -s -- v0.1.0

Environment variables:
  AXUM_TEMPLATE_INSTALL   Installation directory (default: /usr/local/bin)

Supported release assets:
  - linux-x86_64
  - darwin-x86_64
  - darwin-aarch64
  - windows-x86_64
EOF
    exit 0
fi

main() {
    platform=$(detect_platform)
    install_binary "$VERSION" "$platform"
}

main
