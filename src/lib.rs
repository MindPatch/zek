pub mod utils;
use utils::py::pyrunner::PyRunner;

pub fn lib() {
    let req = PyRunner::new();
    let _ = req.run_script("print('Hi')");
}
