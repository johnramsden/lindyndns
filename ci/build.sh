#!/bin/sh

set -ex

# rustup path
export PATH="$PATH:$HOME/.cargo/bin"

install_rustup() {
    curl https://sh.rustup.rs -sSf \
      | sh -s -- -y --default-toolchain="$TRAVIS_RUST_VERSION"
    rustc -V
    cargo -V
}

install_targets() {
    if [ "$(host)" != "${TARGET}" ]; then
        rustup target add "${TARGET}"
    fi
}

main() {
    CARGO="cargo"

    install_rustup
    install_targets

    "${CARGO}" build --target "${TARGET}" --release --color=always --verbose
}

main
