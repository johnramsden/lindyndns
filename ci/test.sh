#!/bin/sh

set -o errexit

create_config() {
    config="${1}"
    cat << EOF > "${config}"
api_token = "${API_KEY}"
domain = "${TRAVIS_OS_NAME}${TRAVIS_COMMIT}.${DOMAIN}"
soa_email = "${EMAIL}"
EOF
}

linux() {
    config_prefix="${HOME}/.config/lindyndns"
    mkdir -p "${config_prefix}"
    create_config "${config_prefix}/config.toml"
}

osx() {
    plist='ca.johnramsden.lindyndns.plist'
    launch_daemons='/Library/LaunchDaemons'

    sudo cp "packaging/macos/${plist}" "${launch_daemons}"
    sudo launchctl load "${launch_daemons}/${plist}"

    config_prefix="${HOME}/Library/Preferences/lindyndns"
    mkdir -p "${config_prefix}"

    create_config "${config_prefix}/config.toml"
}

windows() {
    config_prefix="${LOCALAPPDATA}"'\lindyndns'
    mkdir -p "${config_prefix}"
    create_config "${config_prefix}"'\config.toml'
}

test() {
    "${TRAVIS_OS_NAME}"
    "./target/${TARGET}/release/${PROJECT_NAME}"
}

test
