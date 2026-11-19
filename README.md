## iopolygon

`iopolygon` is a command-line interface (CLI) tool that converts 
test case datasets from an input/output format into an input/solution 
format compatible with the [Polygon](polygon.codeforces.com) system.

## How to use

1. Clone this project to your local.
```
git clone https://github.com/hieuft/iopolygon
cd iopolygon
```
2. Place your test cases—formatted with the required input/output 
structure—into the `./data` directory located at the root of the project.
3. Run `iopolygon`:
```
cargo build
cargo run
```
4. The results will be saved in the `./output` directory at the root of 
the project.
