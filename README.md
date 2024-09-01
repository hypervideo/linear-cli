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

### Command: init

```
Initialize the configuration of `lr`. Will prompt for the API key and write $XDG_CONFIG_HOME/linear-cli/config.toml

Usage: lr init

Options:
  -h, --help  Print help
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

Usage: lr list [OPTIONS]

Options:
  -n, --limit <N>
  -s, --sort-by <SORT_BY>      [default: created] [possible values: created, updated]
  -a, --assignee <ASSIGNEE>
      --state <STATE>          [possible values: started, unstarted, backlog, completed, canceled]
      --not-state <NOT_STATE>  [possible values: started, unstarted, backlog, completed, canceled]
      --json
      --full-width
  -h, --help                   Print help
```

#### Examples

List todo items assigned to a user, sorted by updated date:

```
lr list --not-state completed,canceled --assignee robert --sort-by updated
```
