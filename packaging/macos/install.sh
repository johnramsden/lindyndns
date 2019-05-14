#!/bin/bash

# Temp installer until "proper" package made http://bomutils.dyndns.org/tutorial.html

api="https://api.github.com/repos"
#repo="johnramsden/lindyndns"
repo="mozilla/geckodriver"
ver="latest"
platform="macos"

plist='ca.johnramsden.lindyndns.plist'
launch_daemons='/Library/LaunchDaemons'

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit 1
fi

get_release() {
    curl --silent "${api}/${repo}/releases/${ver}" \
      | grep browser_download_url \
      | grep "${platform}" \
      | cut -d '"' -f 4
}

install() {
    rel=$(get_release)

    echo "Downloading ${rel}"
    tmp="$(mktemp -d)"
    cd "${tmp}"
    wget "${rel}"

    name="${rel##*/}"
    tar xvf "${name}"

    name_dir="${name%.tar.gz}"
    find . # ${name_dir}
    cp "${name_dir}/lindyndns" '/usr/bin/lindyndns'
    chmod +x '/usr/bin/lindyndns'
    cp "${name_dir}/${plist}" "${launch_daemons}"

    launchctl load "${plist}"

    cd
    rm -rf "${tmp}"
}

