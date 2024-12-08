# Polyglot Advent of Code solution collection

Days are encoded by %y%d, 2015-12-18 becomes 1518.

All implementations take the input from stdin and print the two solutions for
the two parts on separate lines of stdout.

Advent of Code is at https://adventofcode.com/

## Running a solution

Use the `aoc` runner script with your language suffix and date, for a Python
solution for 2015-12-01, you'd use

    ./aoc run py/1501

The runner expects to find the corresponding personal input at `input/1501.txt`.

## Automatic interaction with `adventofcode.com`

Get your `session` cookie from a logged-in Advent of Code browser session and
store it in `ADVENT_SESSION` environment variable. When this variable is set,
the `aoc` tool can download personal inputs from the website and to submit
solutions. Eg.

    export ADVENT_SESSION=***YOUR-COOKIE-STRING-HERE***
    ./aoc get 1501

    # ...write a C solution...

    ./aoc submit c/1501

## Writing a new solution

* Use `aoc get` or save your personal input manually in a file named according
  to the year and the day in the `input/` subdirectory, 2018-12-14 becomes
  `input/1814.txt`.

* You can add one or more examples in a file with the day's prefix followed by
  any non-numeric characters, eg. `input/1814a.txt`. The example file can have
  multiple regions of inputs and expected answers separated by `%` alone in a
  line.

  Example file with input and expected outputs:

    ULL
    RRDDD
    LURDL
    UUUUD
    %
    1985
    5DB3

  The expected results can have a single line only if you're still working on
  P1, and if you only have a value for P2 you can use `-` for the first
  expected result to ignore it.

* Write your solution using the same naming convention under the directory of
  the language you're using. Details may vary by language. For Python, it's
  `py/a1814.py`. The `a` prefix is included so that the module can be loaded
  into the Python REPL for interactive testing.

* Each language requires a custom `Justfile` that runs the example with the
  operations needed for that language. The format for running is

      just run [DAY]

  eg.

      just run 1814

  in the subdirectory of the language.

  You should also provide `just exists [DAY]` that will return an error code
  if no program exists for this day in this language. This allows building
  tools that run through all solutions for a given language automatically.
