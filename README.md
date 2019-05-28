# lindyndns

[![Build Status](https://travis-ci.com/johnramsden/lindyndns.svg?token=4X1vWwTyHTHCUwBTudyN&branch=master)](https://travis-ci.com/johnramsden/lindyndns)

Linode Dynamic DNS Client

## Linode Setup

Create a linode account for the user. For added security, create the domain they will be using
and give them only access to that domain.

Have the user [create an API key](https://cloud.linode.com/profile/tokens) with domain read/write and IP read scopes, or give them access to an API key that has read/write permissions for domains.

To allow `lindyndns` to update, but not create a domain, create the domain and give the user access to update only the one domain they need access to.

## Configuration

Create a `config.toml`, (with each `<VARIABLE>` replaced):

```toml
api_token = "<API_KEY>"
domain = "<DOMAIN>"
soa_email = "<EMAIL>"
```

### Configuration file locations:

User:

* *Linux*
    * `$XDG_CONFIG_HOME/lindyndns/config.toml`
* *MacOS*
    * `~/Library/Preferences/lindyndns/config.toml`
* *Windows*
    * `%APPDATA%\lindyndns\config.toml`

System wide:

* *Linux*
    * `/etc/xdg/lindyndns/config.toml`
* *MacOS*
    * `/Library/Preferences/lindyndns/config.toml`
* *Windows*
    * `%APPDATA%\lindyndns\config.toml`

## Usage

Running `lindyndns` will create the specified domain if the user has permissions to create a domain, otherwise an error message will be printed.

If all permissions are correct, running `lindyndns` will update the specified domain, and print the IP address of the user.

## Installing

Several pre-built packages are available depending on the platform under the [releases page](https://github.com/johnramsden/lindyndns/releases).

*  Linux: 
    * tarball
    * `.deb` package
*  MacOS
    * tarball
    * `.pkg`
*  Windows
    * zip archive
    * `.exe` installer

## License

This project is licensed under the [MIT License](LICENSE).
