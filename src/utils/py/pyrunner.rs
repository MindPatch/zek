use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub struct PyRunner;

impl PyRunner {
    /// Initializes a new PyRunner instance.
    pub fn new() -> Self {
        PyRunner
    }

    /// Runs a Python script with error handling.
    pub fn run_script(&self, script: &str) -> PyResult<()> {
        Python::with_gil(|py| {
            match py.run(script, None, None) {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    e.print(py);
                    Err(e)
                }
            }
        })
    }

    /// Executes Python code with provided variables.
    pub fn run_with_vars(&self, script: &str, vars: &[(&str, i32)]) -> PyResult<()> {
        Python::with_gil(|py| {
            let locals = vars.into_py_dict(py);
            match py.run(script, None, Some(&locals)) {
                Ok(_) => Ok(()),
                Err(e) => {
                    e.print(py);
                    Err(e)
                }
            }
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_script() {
        let runner = PyRunner::new();
        assert!(runner.run_script("print('Hello from test!')").is_ok());
    }

    #[test]
    fn test_run_with_vars() {
        let runner = PyRunner::new();
        let vars = [("x", 10), ("y", 20)];
        assert!(runner.run_with_vars("print(f'The sum of {x} and {y} is {x + y}')", &vars).is_ok());
    }

}
