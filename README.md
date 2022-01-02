A command line interface for interacting with Advent of Code, implemented in Rust.

## Commands

`aoc auth <user> <pass>`

To be implemented. (Need to figure out how to make the proper OAuth requests.) In the meantime, authentication token can be set manually with:

```
aoc config auth-token <token>
```

`aoc config <name> [value]`

Get or set configuration values. (If `value` is provided, updates the specified configuration; otherwise, returns its current value.)

Recognized configurations:

- `auth-token`: Authentication token to use when making API requests
- `input-dir`: Directory where fetched input files are saved

`aoc fetch-input <year> <day>`

Fetches the input file for the given day.
