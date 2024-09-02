# linear client

A simple cli utility to interact with the linear API.


## Usage

Get an API key from the linear app and set it as an environment variable `LINEAR_API_KEY`.

```
Usage: lr [OPTIONS] <COMMAND>

Commands:
  init   Initialize the configuration of `lr`. Will prompt for the API key and write $XDG_CONFIG_HOME/linear-cli/config.toml
  me     Show information about the authenticated user
  team
  issue
  debug
  help   Print this message or the help of the given subcommand(s)

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

### Command: team

```
Usage: lr team <COMMAND>

Commands:
  list  List teams
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


### Command: issue

```
Usage: lr issue <COMMAND>

Commands:
  show    Show details about a single issue
  list    List issues
  update  List issues
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### Examples

List todo items assigned to a user, sorted by updated date:

```
lr issue list --not-state completed,canceled --assignee robert --sort-by updated
```
