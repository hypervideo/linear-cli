# linear client

A simple cli utility to interact with the linear API.


## Usage

Get an API key from the linear app and set it as an environment variable `LINEAR_API_KEY`.

```

Usage: lr --api-key <API_KEY> <COMMAND>

Commands:
  me
  list
  help  Print this message or the help of the given subcommand(s)

Options:
      --api-key <API_KEY>  [env: LINEAR_API_KEY]
  -h, --help               Print help
```

### Command: me

```
Show information about the authenticated user

Usage: lr --api-key <API_KEY> me [OPTIONS]

Options:
      --json  
  -h, --help  Print help
```

### Command: list

```
List issues

Usage: lr --api-key <API_KEY> list [OPTIONS]

Options:
  -n, --n <N>  [default: 10]
      --json   
  -h, --help   Print help
```