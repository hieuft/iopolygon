`iopolygon` is a command-line interface (CLI) tool that converts 
test case datasets from an input/output format into an input/solution 
format compatible with the [Polygon](polygon.codeforces.com) system.

You are welcome to take on this project, modify the source code as 
you see fit, and use it.

If you encounter any bugs or have ideas for improvements, 
please feel free to open an [issue](https://github.com/hieuft/iopolygon/issues) 
or submit a [pull request](https://github.com/hieuft/iopolygon/pulls).

## Currently supported test case formats

An input file can have an extension such as .i, .in, or .inp, 
provided that its name contains “.i”.

Similarly, an output file can have an extension such as .o or .out, 
provided that its name contains “.o”.

```
data/
    test01/
        test01.inp
        test01.out
    test02/
        test02.inp
        test02.out
    ...
    testXX/
        testXX.inp
        testXX.out
```

Other test case formats will be added in the future.

## How it work

iopolygon recursively traverses the `./data` directory.
Whenever it enters a directory, it treats it as a test case folder; 
if it successfully locates the corresponding input and output files, 
it adds a new test case—formatted for use with Polygon—to the `./output` 
directory. 

```
output/
    01
    02
    ...
    XX
```

`01` content:
```
<input of testcase>
THISISSEPARATOR
<output of testcase>
```

`solution.cpp` will serve as a mock solution; it will read the input until 
it encounters the keyword `THISISSEPARATOR`, and then produce an output 
identical to the one shown below.

## Current drawbacks

1. For problems that require reading input until the end 
(i.e., until the input stream is exhausted), errors may occur; 
if a user's solution reads the entire input, it will also consume 
the content following the `THISISSEPARATOR` marker.
2. There is a risk of leaking test cases, so please exercise caution 
if considering the use of `iopolygon` for a competition.

## How to use

1. Clone this project to your local.
```
git clone https://github.com/hieuft/iopolygon
cd iopolygon
```
2. Place your test cases—input/output structure—into the `./data` 
directory located at the root of the project.
3. Run `iopolygon`:
```
cargo build
cargo run
```

Input only (no output, and `THISISSEPARATOR` line):
```
cargo run -- no-output
```
4. The results will be saved in the `./output` directory at the root of 
the project.
