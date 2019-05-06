#!/bin/sh

linux() {
    true
}

osx() {
    plist='ca.johnramsden.lindyndns.plist'
    launch_daemons='/Library/LaunchDaemons'

    sudo cp "target/${TARGET}/release/${PROJECT_NAME}" '/usr/bin/lindyndns'
    sudo chmod +x '/usr/bin/lindyndns'
    sudo cp "packaging/macos/${plist}" "${launch_daemons}"

    sudo launchctl load "${launch_daemons}/${plist}"

    mkdir -p ~/Library/Preferences/lindyndns/
    cat << EOF > ~/Library/Preferences/lindyndns/config.toml
api_token = \"${API_KEY}\"
domain = \"${TRAVIS_OS_NAME}.${DOMAIN}\"
soa_email = \"${EMAIL}\"
EOF
}

windows() {
    true
}

test() {
    "${TRAVIS_OS_NAME}"
    /usr/bin/lindyndns
}

test
