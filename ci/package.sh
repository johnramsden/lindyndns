#!/bin/sh

set -e

## Setup

tmpdir="$(mktemp -d)"

if [ -n "${TRAVIS_TAG}" ]; then version="${TRAVIS_TAG}"; else version="0.0.0"; fi

name="${PROJECT_NAME}-${version}-${TARGET}"
staging="${tmpdir}/${PROJECT_NAME}"

out_dir="$(pwd)/deployment"
mkdir -p "${out_dir}" "${staging}"

##

linux() {
    cp "packaging/linux/${PROJECT_NAME}.service" "${staging}"
    cp "packaging/linux/${PROJECT_NAME}.timer" "${staging}"

    (cd "${tmpdir}" && tar czf "${out_dir}/${name}.tar.gz" "${PROJECT_NAME}")

    # Build deb
    cargo install cargo-deb
    cargo deb --output "${out_dir}"
}

osx() {
    git clone https://github.com/munki/munki-pkg.git munkipkg
    cd munkipkg

    cp -r "${TRAVIS_BUILD_DIR}/packaging/macos/munkipkg/lindyndns" lindyndns
    echo "version: '${version}'" >> lindyndns/build-info.yaml

    payload="lindyndns/payload"
    plist='ca.johnramsden.lindyndns.plist'

    bindir="${payload}/usr/local/bin"
    launch_daemons="${payload}/Library/LaunchDaemons"

    mkdir -p "${bindir}" "${launch_daemons}"

    cp "${staging}/lindyndns" "${bindir}"
    cp "${TRAVIS_BUILD_DIR}/packaging/macos/${plist}" "${launch_daemons}"

    (
        python3 -m venv venv && . venv/bin/activate && \
        pip install -r requirements.txt && \
        python3 munkipkg lindyndns
    )

    cp lindyndns/build/* "${out_dir}"

    (cd "${tmpdir}" && tar czf "${out_dir}/${name}.tar.gz" "${PROJECT_NAME}")
}

windows() {
    echo "Installing nsis"
    choco install nsis --yes

    cp -r "packaging/windows/nsis" "${tmpdir}"

    (
        cd "${tmpdir}"
        cp "${PROJECT_NAME}/"* "nsis"

        7z a -tzip  "${out_dir}/${name}.zip" "${PROJECT_NAME}"
        
        cd "nsis"
        makensis.exe "installer_${TARGET_ARCH}.nsi"
        cp "lindyndns_setup_${TARGET_ARCH}.exe" "${out_dir}/lindyndns-${version}_setup_${TARGET_ARCH}.exe"
    )

}

main() {
    cp "target/${TARGET}/release/${PROJECT_NAME}" "${staging}/"

    "${TRAVIS_OS_NAME}"

    rm -rf "${tmpdir}"
}

main

