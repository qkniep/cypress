# cypress

Attach a progress bar to any CLI tool.

## Usage

Cypress supports progress by number of lines:

```sh
ping ::1 -c 10 | cypress --lines --max 10
```

Alternatively, you can transform the output into a stream of integers and provide the `--min` and `--max` options:

```sh
ping ::1 -c 10 | sed 's/.*seq=\([0-9]\+\).*/\1/' | cypress --min 1 --max 10
```

Note that in this example the piped data contains a non-integer (empty string) line.
This is ignored by Cypress, as would values outside the `--min` and `--max` range.
