use pyo3::{
    pyfunction, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyAny, PyResult,
};

const PYSNAPSHOT_SUFFIX: &str = "pysnap";

fn setttings(snapshot_path: String) -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(format!("{snapshot_path}/snapshots"));
    settings.set_snapshot_suffix(PYSNAPSHOT_SUFFIX);
    settings
}

#[pyfunction]
fn assert_json_snapshot(
    snapshot_path: String,
    snapshot_name: String,
    result: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let res: serde_json::Value = pythonize::depythonize(result).unwrap();
    let settings = setttings(snapshot_path);
    settings.bind(|| {
        insta::assert_json_snapshot!(snapshot_name, res);
    });
    Ok(())
}

#[pyfunction]
fn assert_csv_snapshot(
    snapshot_path: String,
    snapshot_name: String,
    result: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let res: serde_json::Value = pythonize::depythonize(result).unwrap();
    let settings = setttings(snapshot_path);
    settings.bind(|| {
        insta::assert_csv_snapshot!(snapshot_name, res);
    });
    Ok(())
}

#[pyfunction]
fn assert_snapshot(
    snapshot_path: String,
    snapshot_name: String,
    result: &Bound<'_, PyAny>,
) -> PyResult<()> {
    let settings = setttings(snapshot_path);
    settings.bind(|| {
        insta::assert_snapshot!(snapshot_name, result);
    });
    Ok(())
}

#[pymodule]
#[pyo3(name = "_pysnaptest")]
fn pysnaptest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(assert_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(assert_json_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(assert_csv_snapshot, m)?)?;
    Ok(())
}
