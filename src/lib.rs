use pyo3::{
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyAny, PyResult,
};

const PYSNAPSHOT_SUFFIX: &str = "pysnap";

#[pyfunction]
fn assert_json_snapshot(
    folder_path: String,
    name: String,
    result: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let res: serde_json::Value = pythonize::depythonize(&result).unwrap();
    insta::with_settings!({snapshot_path => format!("{folder_path}/snapshots"), snapshot_suffix => PYSNAPSHOT_SUFFIX}, {
        insta::assert_json_snapshot!(name, res);
    });
    Ok(())
}

#[pyfunction]
fn assert_snapshot(folder_path: String, name: String, result: &Bound<'_, PyAny>) -> PyResult<()> {
    insta::with_settings!({snapshot_path => format!("{folder_path}/snapshots"), snapshot_suffix => PYSNAPSHOT_SUFFIX}, {
        insta::assert_snapshot!(name, result);
    });
    Ok(())
}

#[pymodule]
#[pyo3(name = "_lib_name")]
fn pysnaptest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(assert_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(assert_json_snapshot, m)?)?;
    Ok(())
}
