A simple CLI tool for wrapping text.

# Usage
```
reptar [OPTIONS] <TEXT_OR_PATH>

Arguments:
  <TEXT_OR_PATH>
          The text to be wrapped or the path to the
          file containing the text to be wrapped
          (must use --file flag for path input)

Options:
  -b, --break-words
          Break lines in the middle of words

  -s, --separator <SEPARATOR>
          Whether to use regular ASCII separators, or
          Unicode separators for the line breaking
          algorithm

          [default: unicode]
          [possible values: ascii, unicode]

  -i, --initial-indent <INITIAL_INDENT>
          The initial indent of the first paragraph

  -w, --width <WIDTH>
          The width of the terminal.

          Possible values are `term`, `TERM_WIDTH%`,
          or an integer.

          [default: term]

  -f, --file
          Whether to interpret the text_or_path
          argument as a file path and wrap the
          lines of its text

  -h, --help
          Print help (see a summary with '-h')
```
