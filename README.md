# Rusted Python - slowly turn Python project into Rust

## Click-bait results

Run the test on your machine:

   pytest slow.py 

Results:

<img alt="Description" src="https://github.com/PavelVavruska/rusted-python/blob/master/results_20220611.png">


## Install virtualenv for Python:

```
python3 -m venv env    # create Python env locally in your project folder
source env/bin/activate   # activate
which python   # what python are you using
deactivate  # leave the local python env
python3 -m pip install -r requirements.txt  # install libs from requirements.txt
```


## If you're using bash (on a Mac or GNU/Linux distro), add this to your ~/.bashrc

    export PYTHONPATH="${PYTHONPATH}:/my/other/path"

Ubuntu: 

    export PYTHONPATH="${PYTHONPATH}:/home/rust/.local/bin"


## How to update rust project part and use it in Python:

Step 1 and 2 is automated by SH script ./build_and_copy_bin.sh

1) build rust project in ./pyext-rustlib
```
cargo build --release
```
2) copy rust library into Python folder
```
/rusted-python$ cp pyext-rustlib/target/release/librustlib.so rustlib.so
```
3) import rustlib # and use it as any other Python library


## FAQ / Common errors:

error: linker `cc` not found
-> 
```
sudo apt-get update
sudo apt install build-essential
```