from rustlib import multiply_rust  # rust version

def multiply(a: int, b: int):
    return a * b

def run_pure_python():
    multiply(1,5)

def run_imported_rust():
    multiply_rust(1,5)

def test_pure_python(benchmark):  
    # comment out if pytest is not installed
    benchmark(run_pure_python)

def test_imported_rust(benchmark):  
    # comment out if pytest is not installed
    benchmark(run_imported_rust)

if __name__ == "__main__":
    print("slow python: {}".format(multiply(1,5)))
    print("fast rust: {}".format(multiply_rust(1,5)))
