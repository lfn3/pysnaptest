use std::path::PathBuf;
use std::str;
use std::{env, path::Path};

use pyo3::{
    exceptions::PyValueError,
    pyclass, pyfunction, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction, Bound, PyAny, PyErr, PyResult,
};

const PYSNAPSHOT_SUFFIX: &str = "pysnap";

#[derive(Debug)]
struct Description {
    test_file_path: String,
}

impl Description {
    pub fn new(test_file_path: String) -> Self {
        Self { test_file_path }
    }
}

impl Into<String> for Description {
    fn into(self) -> String {
        format!("Test File Path: {}", self.test_file_path)
    }
}


struct PytestStr(String);

struct PytestInfo {
    test_path: String,
    pub test_name: String,
    _test_stage: String,
}

impl PytestInfo {
    pub fn test_path(&self) -> PyResult<PathBuf> {
        let path = self.test_path_raw();
        if path.exists() {
            Ok(path)
        } else {
            if let Some(filename) = path.file_name() {
                let mut filepath = PathBuf::from("./");
                filepath.push(filename);
                Ok(filepath)
            } else {
                return Err(PyValueError::new_err("No test file found"));
            }
        }
    }

    pub fn test_path_raw(&self) -> PathBuf {
        Path::new(&self.test_path).to_path_buf()
    }
}

impl PytestStr {
    pub fn from_env() -> Self {
        Self(env::var("PYTEST_CURRENT_TEST").expect("PYTEST_CURRENT_TEST should be set"))
    }
}

impl TryInto<PytestInfo> for PytestStr {
    type Error = PyErr;
    fn try_into(self) -> Result<PytestInfo, Self::Error> {
        let re = regex::Regex::new(r"^(?P<test_path>(?:\/tests\/[^\/]+|tests\/[^\/]+)+\.py)::(?P<test_name>[\w_]+)\s\((?P<test_stage>setup|call|teardown)\)$").expect("Regex should be valid");
        let Some(caps) = re.captures(&self.0) else {
            return Err(PyValueError::new_err(format!(
                "PYTEST_CURRENT_TEST does not match expected format {}",
                &self.0
            )));
        };
        Ok(PytestInfo {
            test_name: caps["test_name"].to_string(),
            test_path: caps["test_path"].to_string(),
            _test_stage: caps["test_stage"].to_string(),
        })
    }
}

#[pyclass(frozen)]
struct TestInfo {
    pytest_info: PytestInfo,

    snapshot_path_override: Option<PathBuf>,
    snapshot_name_override: Option<String>,
}

#[pymethods]
impl TestInfo {
    #[staticmethod]
    #[pyo3(signature = (snapshot_path_override = None, snapshot_name_override = None))]
    fn from_pytest(
        snapshot_path_override: Option<PathBuf>,
        snapshot_name_override: Option<String>,
    ) -> PyResult<Self> {
        let pytest_test_str = PytestStr::from_env();
        let pytest_info: PytestInfo = pytest_test_str.try_into()?;
        Ok(TestInfo {
            pytest_info,
            snapshot_path_override,
            snapshot_name_override,
        })
    }

    fn snapshot_path(&self) -> PyResult<PathBuf> {
        if let Some(snapshot_path) = self.snapshot_path_override.clone() {
            return Ok(snapshot_path);
        }

        let mut test_file_dir = self
            .pytest_info
            .test_path()?
            .canonicalize()?
            .parent()
            .ok_or_else(|| {
                PyValueError::new_err(format!(
                    "Invalid test_path: {:?}, not yielding a parent directory",
                    self.pytest_info.test_path_raw()
                ))
            })?
            .to_path_buf();
        test_file_dir.push("snapshots");

        Ok(test_file_dir)
    }

    fn snapshot_name(&self) -> String {
        if let Some(sno) = self.snapshot_name_override.as_ref() {
            return sno.clone();
        }

        let test_name = &self.pytest_info.test_name;
        let test_path = self.pytest_info.test_path_raw();
        let file_name = test_path.file_stem().and_then(|s| s.to_str());
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
        settings.set_description(
            Description::new(self.pytest_info.test_path()?.to_string_lossy().to_string()),
        );
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
