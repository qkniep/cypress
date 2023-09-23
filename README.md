# cypress

Turn (almost) any streaming output into a progress bar.

## Basic Usage

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

If you are having trouble getting the correct regex working ([we've all been there](https://xkcd.com/1171)),
try adding the debug option for additional info on what happens during parsing:

```sh
ping ::1 -c 10 | cypress --min 1 --max 10 --regex 'seq=(\d+)' --debug
```

## Further Examples

Popular intermediate formats can easily be adapted by using an existing processor for the repsective format.
Some popular examples include:

### JSON

Here we will use the [`jq`](https://github.com/jqlang/jq) command line JSON processor:

```sh
./test_json.sh | jq '.id' --unbuffered | cypress --min 1 --max 10
```

### CSV

Here we will use the [`qsv`](https://github.com/jqnatividad/qsv) command line CSV processor,
a fork of the more popular [`xsv`](https://github.com/BurntSushi/xsv):

```sh
export QSV_RDR_BUFFER_CAPACITY=1
export QSV_WTR_BUFFER_CAPACITY=1
./test_csv.sh | qsv select id | cypress --min 1 --max 10
```

Setting the `QSV_RDR_BUFFER_CAPACITY` and `QSV_WTR_BUFFER_CAPACITY` to 1 byte each will make `qsv` stream line by line without unwanted buffering.
