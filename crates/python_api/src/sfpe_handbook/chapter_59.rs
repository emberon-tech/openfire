pub mod equation_59_10;
pub mod equation_59_11;
pub mod equation_59_12;
pub mod equation_59_13;
pub mod equation_59_15;
pub mod equation_59_5;
pub mod equation_59_6;
pub mod equation_59_7;
pub mod equation_59_8;
pub mod equation_59_9;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
pub fn chapter_59(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(equation_59_5::equation_59_5))?;
    m.add_wrapped(wrap_pymodule!(equation_59_6::equation_59_6))?;
    m.add_wrapped(wrap_pymodule!(equation_59_7::equation_59_7))?;
    m.add_wrapped(wrap_pymodule!(equation_59_8::equation_59_8))?;
    m.add_wrapped(wrap_pymodule!(equation_59_9::equation_59_9))?;
    m.add_wrapped(wrap_pymodule!(equation_59_10::equation_59_10))?;
    m.add_wrapped(wrap_pymodule!(equation_59_11::equation_59_11))?;
    m.add_wrapped(wrap_pymodule!(equation_59_12::equation_59_12))?;
    m.add_wrapped(wrap_pymodule!(equation_59_13::equation_59_13))?;
    m.add_wrapped(wrap_pymodule!(equation_59_15::equation_59_15))?;
    Ok(())
}
