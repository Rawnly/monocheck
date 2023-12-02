# monocheck

`monocheck` is a command-line tool that checks the dependencies in your monorepo to see if there are duplicates.

## Installation

To install `monocheck`, run the following command:

```
$ cargo install monocheck
```

## Usage

```
$ monocheck [OPTIONS]
```

## Options

* `-i`, `--ignore`: Ignore matching package names.
* `-m`, `--matches`: Filter by matching package name.
* `--min`: Minimum number of workspaces to include [default: 2].
* `-I`, `--ignore-workspace`: Ignore matching workspaces names.
* `-M`, `--match-workspace`: Filter by matching workspace name.
* `--deep`: Check for version differences in dependencies.
* `--json`: Output as JSON.
* `-h`, `--help`: Print help.
* `-V`, `--version`: Print version.

When the `--deep` flag is used, `monocheck` will check for version differences in dependencies. When `--json` is used, the output will be in JSON format.

The `--ignore`, `--matches`, `--ignore-workspace`, and `--match-workspace` options accept regular expressions, which can be useful for more complex matching criteria.

The default behavior of `monocheck` is to only include workspaces that are depended on by at least 2 other workspaces (as specified by the `--min` option).

## Example

```
$ monocheck --deep --json
{
  "dependencies": [
    {
      "name": "foo",
      "versions": [
        "1.0.0",
        "1.0.1"
      ],
      "workspaces": [
        "workspace1",
        "workspace2",
        "workspace3"
      ]
    },
    {
      "name": "bar",
      "versions": [
        "2.0.0",
        "2.1.0",
        "2.2.0"
      ],
      "workspaces": [
        "workspace1",
        "workspace3"
      ]
    }
  ]
}
```

This command checks for version differences in dependencies and prints the output in JSON format. The output shows the dependencies that are duplicated across different workspaces.
