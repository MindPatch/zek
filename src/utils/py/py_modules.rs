use pyo3::prelude::*;
use pyo3::types::PyDict;
use reqwest::blocking::Client;

#[pyclass]
pub struct HttpClient {
    client: Client,
}

#[pymethods]
impl HttpClient {
    #[new]
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    pub fn get(&self, url: String, headers: Option<&PyDict>) -> PyResult<String> {
        let mut request = self.client.get(&url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                let key: &str = key.extract()?;
                let value: &str = value.extract()?;
                request = request.header(key, value);
            }
        }

        let response = request.send().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Request failed: {}", e)))?;
        let text = response.text().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read response body: {}", e)))?;

        Ok(text)
    }

    pub fn post(&self, url: String, body: Option<String>, headers: Option<&PyDict>) -> PyResult<String> {
        let mut request = self.client.post(&url);

        if let Some(headers) = headers {
            for (key, value) in headers {
                let key: &str = key.extract()?;
                let value: &str = value.extract()?;
                request = request.header(key, value);
            }
        }

        if let Some(body) = body {
            request = request.body(body);
        }

        let response = request.send().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Request failed: {}", e)))?;
        let text = response.text().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read response body: {}", e)))?;

        Ok(text)
    }
}

#[pymodule]
fn http_client(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HttpClient>()?;
    Ok(())
}
