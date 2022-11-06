#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn multiply_rust(_py: Python, a: i32, b: i32) -> PyResult<i32> {
    let mut total = a * b;
    for _ in 0..1000 {
        for _ in 0..1000 {
            for _ in 0..100 {
                total+=1
            }
        }
    }
    
    Ok(total)
}

py_module_initializer!(rustlib, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "multiply_rust", py_fn!(py, multiply_rust(a: i32, b: i32)))?;
    Ok(())
});