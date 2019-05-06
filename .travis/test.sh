#!/bin/sh

linux() {
    mkdir -p ~/.config/lindyndns/
    cat << EOF > ~/.config/lindyndns/config.toml
api_token = "${API_KEY}"
domain = "${TRAVIS_OS_NAME}.${DOMAIN}"
soa_email = "${EMAIL}"
EOF
}

osx() {
    plist='ca.johnramsden.lindyndns.plist'
    launch_daemons='/Library/LaunchDaemons'

    sudo cp "packaging/macos/${plist}" "${launch_daemons}"

    sudo launchctl load "${launch_daemons}/${plist}"

    mkdir -p ~/Library/Preferences/lindyndns/
    cat << EOF > ~/Library/Preferences/lindyndns/config.toml
api_token = "${API_KEY}"
domain = "${TRAVIS_OS_NAME}.${DOMAIN}"
soa_email = "${EMAIL}"
EOF
}

windows() {
    true
}

test() {
    "${TRAVIS_OS_NAME}"
    "./target/${TARGET}/release/${PROJECT_NAME}"
}

test
