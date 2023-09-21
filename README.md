# cypress

Turn (almost) any streaming output into a progress bar.

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

Also, cypress supports regex parsing out of the box:

```sh
ping ::1 -c 10 | cypress --min 1 --max 10 --regex 'seq=(\d+)'
```

If you are having trouble getting the correct regex working, try adding the debug option for additional info on what happens during parsing:

```sh
ping ::1 -c 10 | cypress --min 1 --max 10 --regex 'seq=(\d+)' --debug
```
