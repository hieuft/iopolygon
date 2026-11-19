`iopolygon` is a command-line interface (CLI) tool that converts 
test case datasets from an input/output format into an input/solution 
format compatible with the [Polygon](polygon.codeforces.com) system.

You are welcome to take on this project, modify the source code as 
you see fit, and use it.

If you encounter any bugs or have ideas for improvements, 
please feel free to open an issue or submit a pull request.

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
directory. The resulting test cases will follow this structure:

```
output/
    01
    02
    ...
    XX
```

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
4. The results will be saved in the `./output` directory at the root of 
the project.
