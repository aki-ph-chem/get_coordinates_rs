# coordinates_picker.rs

A tool to extract the optimized coordinates from Gaussian \*LOG files implemented in Rust.

## Installation

Clone the source code and build with `Cargo`:

```bash
$ git clone git@github.com:aki-ph-chem/get_coordinates_rs.git
$ cd coordinates_picker
$ cargo build --release
```

The built binary will be located at `target/release/coordinates_picker`. Move it to a directory in your path.

For Arch-based Linux users, there is a script for packaging `build_pacman` ([here](./build_pacman)). Use it to build and install the package.

Here is an example using the helper script `build.sh`:

```bash
$ cd build_pacman 
$ ./build.sh
```

For Debian-based Linux distribution user, there is a script for packaging `build_dpkg` ([here](./build_dpkg)). Use it to build and install the package.

```bash
$ cd build_dpkg
$ ./build.sh
```
If you want to perform container-based builds, you can use `build_pacman/config`
for Arch-based system or the `build_dpkg/config` 
for Debian-based systems, which contain scripts to set up the container environment.

## Usage

If you installed using `build_pacman`, the command name available is `pick_co`. (This may be different if you installed in a different way)

The command is used as follows, where the first argument is the Gaussian \*.LOG file, the second is the number of atoms in the molecule you want to extract, and the third is the name of the file to write the results to:

```bash
$ pick_co <file name of input> <numbers of atoms> <file name of output>
```

### Example

Below is an example usage of the `pick_co` command using the sample data located in `coordinates_picker/for_test`.

```bash
$ pick_co test_input.LOG 12 output.csv 
```

After running this command, a file named `output.csv` will be generated in the current directory.

If you examine the contents of this file, you will see that it is a csv file with the following format:

```bash
$ cat gomi.csv
1,6,0,-0.042214,1.387483,-0.124879
2,6,0,-1.373387,0.677396,0.071920
3,6,0,-1.373402,-0.677370,0.071909
4,6,0,-0.042245,-1.387488,-0.124878
5,1,0,0.021754,1.869823,-1.110964
6,1,0,0.144642,2.156219,0.637581
7,1,0,-2.286675,1.266023,0.171891
8,1,0,-2.286704,-1.265977,0.171869
9,1,0,0.021727,-1.869859,-1.110946
10,1,0,0.144590,-2.156201,0.637611
11,15,0,1.320856,-0.000008,-0.013380
12,1,0,1.415322,-0.000024,1.439234
```

In version `1.1`, if no output file name is given as the third argument, the result will be printed to standard output.

```bash
$ pick_co test_input.LOG 12
1,6,0,-0.042214,1.387483,-0.124879
2,6,0,-1.373387,0.677396,0.071920
3,6,0,-1.373402,-0.677370,0.071909
4,6,0,-0.042245,-1.387488,-0.124878
5,1,0,0.021754,1.869823,-1.110964
6,1,0,0.144642,2.156219,0.637581
7,1,0,-2.286675,1.266023,0.171891
8,1,0,-2.286704,-1.265977,0.171869
9,1,0,0.021727,-1.869859,-1.110946
10,1,0,0.144590,-2.156201,0.637611
11,15,0,1.320856,-0.000008,-0.013380
12,1,0,1.415322,-0.000024,1.439234
```

## License

MIT License
