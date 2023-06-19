# jvm-rs
it's (a little bit of) a (slow) jvm written in rust!

# running
this requires cargo and javac to be installed
```sh
git clone "https://github.com/kwzuu/jvm-rs.git" && cd jvm-rs  # clone the repository and enter it
cd test  # go into test directory
./build_test_classes.sh  # build test cases
cd class  # go into classfiles directory
cargo run Iteration.class  # run the program! the return value should be 362880 in the iterative factorial demo
```
