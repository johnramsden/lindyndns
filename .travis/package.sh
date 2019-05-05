#!/bin/sh

linux() {
    cp "packaging/linux/${PROJECT_NAME}.service" "${staging}"
    cp "packaging/linux/${PROJECT_NAME}.timer" "${staging}"
}

osx() { true; }
windows() { true; }

main() {
    tmpdir="$(mktemp -d)"
    tag=""
    if [ -n "${TRAVIS_TAG}" ]; then tag="-${TRAVIS_TAG}"; fi
    name="${PROJECT_NAME}${tag}-${TARGET}"
    staging="${tmpdir}/${name}"

    out_dir="$(pwd)/deployment"
    mkdir -p "${out_dir}" "${staging}"

    find "target/${TARGET}/release/"

    cp "target/${TARGET}/release/${PROJECT_NAME}" "${staging}/"

    "${TRAVIS_OS_NAME}"

    (cd "${tmpdir}" && tar czf "${out_dir}/${name}.tar.gz" "${name}")
    rm -rf "${tmpdir}"
}

main
