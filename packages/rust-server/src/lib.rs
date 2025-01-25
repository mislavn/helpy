use common::{Calculator, CalculatorInput, MyResponse};
use my_logging::init_logging;
use opentelemetry::trace::Tracer;
use opentelemetry::{global, trace::Span};
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use snafu::prelude::*;
use std::sync::LazyLock;

mod error;

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to start async runtime!")
});

fn create_span(name: &str, trace_id: u128, span_id: u64) -> global::BoxedSpan {
    let tracer = global::tracer("PyServer");
    tracer
        .span_builder(name.to_string())
        .with_trace_id(trace_id.into())
        .with_span_id(span_id.into())
        .start(&tracer)
}

#[pyclass]
#[derive(Clone, Debug)]
struct RustClass {
    calculator: Calculator,
}

#[pymethods]
impl RustClass {
    #[new]
    pub fn new() -> Self {
        RustClass {
            calculator: Calculator::new(),
        }
    }

    pub fn counter(&self, trace_id: u128, span_id: u64) -> Result<usize, error::MyError> {
        let mut span = create_span("counter", trace_id, span_id);
        span.add_event("Rust tracing counter", vec![]);
        log::info!("rust-server: counter");
        self.calculator.counter().context(error::CommonSnafu)
    }

    pub fn sum_array(
        &self,
        py: Python,
        input: PyObject,
        trace_id: u128,
        span_id: u64,
    ) -> Result<PyObject, error::MyError> {
        let mut span = create_span("sum_array", trace_id, span_id);
        span.add_event("Rust tracing sum_array", vec![]);

        tracing::info!("Trace from python bindings sum_array");

        let input: CalculatorInput =
            depythonize(&input.into_bound(py)).context(error::ConversionSnafu)?;
        span.add_event(format!("input: {:?}", input), vec![]);
        log::info!("rust-server: sum_array {:?}", input);
        let result = self
            .calculator
            .sum_array(input)
            .context(error::CommonSnafu)?;
        Ok(pythonize(py, &result)
            .context(error::ConversionSnafu)?
            .into())
    }
}

#[pyfunction]
fn hello_from_rust() -> Result<String, error::MyError> {
    let response = MyResponse {
        status: "success".to_string(),
        message: "hello world from Rust".to_string(),
    };

    serde_json::to_string(&response).context(error::SerdeSnafu)
}

#[pyfunction]
fn rust_error() -> Result<String, error::MyError> {
    Err(error::MyError::Unknown {
        msg: "Error created in Rust".to_string(),
        loc: snafu::location!(),
    })
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    RUNTIME.block_on(async move { init_logging() });
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(hello_from_rust, m)?)?;
    m.add_function(wrap_pyfunction!(rust_error, m)?)?;
    m.add_class::<RustClass>()?;
    Ok(())
}
