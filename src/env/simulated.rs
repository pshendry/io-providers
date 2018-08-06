use std::ffi;
use std::io;
use std::path::{Path, PathBuf};
use std::vec;

use env::Env;

/// Provides inspection and manipulation of a simulated process's environment.
#[derive(Default)]
pub struct SimulatedEnv {
    args: Option<Vec<String>>,
    args_os: Option<Vec<ffi::OsString>>,
    current_dir: Option<PathBuf>,
    current_exe: Option<PathBuf>,
}

impl SimulatedEnv {
    /// Creates a new virtual environment.
    pub fn new() -> SimulatedEnv {
        SimulatedEnv {
            args: None,
            args_os: None,
            current_dir: None,
            current_exe: None,
        }
    }

    /// Sets the arguments which this program was started with (normally passed via the command
    /// line).
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = Some(args);
    }

    /// Sets the arguments which this program was started with (normally passed via the command
    /// line).
    pub fn set_args_os(&mut self, args: Vec<ffi::OsString>) {
        self.args_os = Some(args);
    }

    /// Sets the path to be returned by `Env::current_exe()`.
    pub fn set_current_exe<P: AsRef<Path>>(&mut self, path: P) {
        self.current_dir = Some(PathBuf::from(path.as_ref()));
    }
}

impl Env for SimulatedEnv {
    type ArgsIter = vec::IntoIter<String>;
    type ArgsOsIter = vec::IntoIter<ffi::OsString>;

    fn args(&self) -> Self::ArgsIter {
        self.args.clone()
            .expect("Env::args() was called before a simulated value was set")
            .into_iter()
    }

    fn args_os(&self) -> Self::ArgsOsIter {
        self.args_os.clone()
            .expect("Env::args_os() was called before a simulated value was set")
            .into_iter()
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        Ok(self.current_dir.clone()
            .expect("Env::current_dir() was called before a simulated value was set"))
    }

    fn current_exe(&self) -> io::Result<PathBuf> {
        Ok(self.current_exe.clone()
            .expect("Env::current_exe() was called before a simulated value was set"))
    }

    fn set_current_dir<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        self.current_dir = Some(PathBuf::from(path.as_ref()));
        Ok(())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::ffi::OsString;
    use std::path::Path;
    use super::SimulatedEnv;
    use env::Env;

    #[test]
    #[should_panic]
    fn args__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.args();
    }

    #[test]
    fn args__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let args = vec!["app".to_string(), "arg1".to_string(), "arg2".to_string()];

        provider.set_args(args.clone());
        let result: Vec<String> = provider.args().collect();

        assert_eq!(args, result);
    }

    #[test]
    #[should_panic]
    fn args_os__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.args_os();
    }

    #[test]
    fn args_os__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let args = vec![OsString::from("app"), OsString::from("arg1"), OsString::from("arg2")];

        provider.set_args_os(args.clone());
        let result: Vec<OsString> = provider.args_os().collect();

        assert_eq!(args, result);
    }

    #[test]
    #[should_panic]
    fn current_dir__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.current_dir();
    }

    #[test]
    fn current_dir__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let path = Path::new("/foo/bar");

        provider.set_current_dir(path).unwrap();
        let result = provider.current_dir().unwrap();

        assert_eq!(path, result.as_path());
    }

    #[test]
    #[should_panic]
    fn current_exe__called_before_set__panics() {
        let provider = SimulatedEnv::new();
        let _ = provider.current_exe();
    }

    #[test]
    fn current_exe__set_and_get__success() {
        let mut provider = SimulatedEnv::new();
        let path = Path::new("/foo/bar");

        provider.set_current_exe(path);
        let result = provider.current_dir().unwrap();

        assert_eq!(path, result.as_path());
    }
}