use std::path::PathBuf;

use pyo3::{
    exceptions::PyValueError,
    pyclass, pyfunction, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyAny, PyErr, PyResult,
};

const PYSNAPSHOT_SUFFIX: &str = "pysnap";

#[pyclass(frozen)]
struct TestInfo {
    test_path: PathBuf,
    test_name: String,

    snapshot_path_override: Option<PathBuf>,
    snapshot_name_override: Option<String>,
}

#[pymethods]
impl TestInfo {
    #[new]
    #[pyo3(signature = (test_name, test_path, snapshot_path_override = None, snapshot_name_override = None))]
    fn new(
        test_name: String,
        test_path: PathBuf,
        snapshot_path_override: Option<PathBuf>,
        snapshot_name_override: Option<String>,
    ) -> PyResult<Self> {
        Ok(TestInfo {
            test_name,
            test_path,
            snapshot_path_override,
            snapshot_name_override,
        })
    }

    fn snapshot_path(&self) -> PyResult<PathBuf> {
        if let Some(snapshot_path) = self.snapshot_path_override.clone() {
            return Ok(snapshot_path);
        }

        let mut test_file_dir = self
            .test_path
            .parent()
            .ok_or_else(|| {
                PyValueError::new_err("Invalid 'current_test' value - should contain a single '::'")
            })?
            .to_path_buf();

        test_file_dir.push("snapshots");

        Ok(test_file_dir)
    }

    fn snapshot_name(&self) -> String {
        if let Some(sno) = self.snapshot_name_override.as_ref() {
            return sno.clone();
        }

        let test_name = self
            .test_name
            .strip_suffix(" (call)")
            .unwrap_or(self.test_name.as_ref());
        let file_name = self.test_path.file_stem().and_then(|s| s.to_str());
        if let Some(f) = file_name {
            format!("{f}_{test_name}")
        } else {
            test_name.to_string()
        }
    }
}

impl TryInto<insta::Settings> for &TestInfo {
    type Error = PyErr;

    fn try_into(self) -> PyResult<insta::Settings> {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(self.snapshot_path()?);
        settings.set_snapshot_suffix(PYSNAPSHOT_SUFFIX);
        settings.set_input_file(&self.test_path);
        settings.set_omit_expression(true);
        Ok(settings)
    }
}

#[pyfunction]
fn assert_json_snapshot(test_info: &TestInfo, result: &Bound<'_, PyAny>) -> PyResult<()> {
    let res: serde_json::Value = pythonize::depythonize(result).unwrap();
    let snapshot_name = test_info.snapshot_name();
    let settings: insta::Settings = test_info.try_into()?;
    settings.bind(|| {
        insta::assert_json_snapshot!(snapshot_name, res);
    });
    Ok(())
}

#[pyfunction]
fn assert_csv_snapshot(test_info: &TestInfo, result: &Bound<'_, PyAny>) -> PyResult<()> {
    let res: serde_json::Value = pythonize::depythonize(result).unwrap();
    let snapshot_name = test_info.snapshot_name();
    let settings: insta::Settings = test_info.try_into()?;
    settings.bind(|| {
        insta::assert_csv_snapshot!(snapshot_name, res);
    });
    Ok(())
}

#[pyfunction]
fn assert_snapshot(test_info: &TestInfo, result: &Bound<'_, PyAny>) -> PyResult<()> {
    let snapshot_name = test_info.snapshot_name();
    let settings: insta::Settings = test_info.try_into()?;
    settings.bind(|| {
        insta::assert_snapshot!(snapshot_name, result);
    });
    Ok(())
}

#[pymodule]
#[pyo3(name = "_pysnaptest")]
fn pysnaptest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TestInfo>()?;

    m.add_function(wrap_pyfunction!(assert_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(assert_json_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(assert_csv_snapshot, m)?)?;
    Ok(())
}
