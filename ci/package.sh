#!/bin/sh

set -e

## Setup

tmpdir="$(mktemp -d)"

[ -n "${TRAVIS_TAG}" ] && tag="-${TRAVIS_TAG}" || tag=""

name="${PROJECT_NAME}${tag}-${TARGET}"
staging="${tmpdir}/${PROJECT_NAME}"

out_dir="$(pwd)/deployment"
mkdir -p "${out_dir}" "${staging}"

##

linux() {
    cp "packaging/linux/${PROJECT_NAME}.service" "${staging}"
    cp "packaging/linux/${PROJECT_NAME}.timer" "${staging}"

    (cd "${tmpdir}" && tar czf "${out_dir}/${name}.tar.gz" "${PROJECT_NAME}")
}

osx() {
    git clone https://github.com/munki/munki-pkg.git munkipkg
    cd munkipkg

    cp -r "${TRAVIS_BUILD_DIR}/packaging/macos/munkipkg/lindyndns" lindyndns

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
    (cd "${tmpdir}" && 7z a -tzip  "${out_dir}/${name}.zip" "${PROJECT_NAME}")
}

main() {
    cp "target/${TARGET}/release/${PROJECT_NAME}" "${staging}/"

    "${TRAVIS_OS_NAME}"
    
    rm -rf "${tmpdir}"
}

main
