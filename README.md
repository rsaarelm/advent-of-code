# Polyglot Advent of Code solution collection

Days are encoded by %y%d, 2015-12-18 becomes 1518.

All implementations take the input from stdin and print the two solutions for
the two parts on separate lines of stdout.

Advent of Code is at https://adventofcode.com/

## Running a solution

Use the runner script with your language suffix and date, for a Python
solution for 2015-12-01, you'd use

    ./run py/1501

## Writing a new solution

* Save your input in a file named according to the year and the day in the
  `input/` subdirectory, 2018-12-14 becomes `input/1814.txt`.

* Save test inputs and outputs using the same number prefix followed by any
  non-numeric characters. Test input for 2018-12-14 can become
  `input/1814a.txt` and the corresponding output `input/1814a.out`.

* The output file can have only a single line of output in case you're still
  working with part 1 of the exercise.

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

* Each solution awaits for input from stdin and prints its results for parts 1
  and 2 to stdout as two lines. While only part 1 is finished, the solution
  prints a single line with the part 1 solution and then exits.
