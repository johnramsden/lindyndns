# lindyndns

## Usage

Create a user, For added security, create the domain they will be using
and give them only access to that domain.

Have the user [create API key](https://cloud.linode.com/profile/tokens),
or give them access to an API key that has read/write for domains.

Create `config.ini`, (with each `<VARIABLE>` replaced):

```ini
[DEFAULT]
api_key = <API_KEY>
domain = <DOMAIN>
email = <EMAIL>
```

## License

This project is licensed under the MIT License.

## Reference

* https://developers.linode.com/api/v4/
* https://docs.python.org/3/library/urllib.parse.html
