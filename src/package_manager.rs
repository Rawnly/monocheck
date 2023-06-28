use std::ffi::OsStr;
use std::io;
use std::process::{Command, Output, Stdio};

pub trait PackageManager {
    fn install() -> io::Result<Output>;
    fn remove(pkg: &str) -> io::Result<Output>;
    fn add(pkg: &str) -> io::Result<Output>;
}

pub fn execute<I>(cmd: &str, args: I) -> io::Result<Output>
where
    I: IntoIterator,
    I::Item: AsRef<OsStr>,
{
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub struct PNPM {}
impl PackageManager for PNPM {
    fn install() -> io::Result<Output> {
        execute("pnpm", ["install"])
    }

    fn remove(pkg: &str) -> io::Result<Output> {
        execute("pnpm", ["remove"])
    }

    fn add(pkg: &str) -> io::Result<Output> {
        execute("pnpm", ["add"])
    }
}

pub struct NPM {}

impl PackageManager for NPM {
    fn install() -> io::Result<Output> {
        execute("npm", ["install"])
    }

    fn remove(pkg: &str) -> io::Result<Output> {
        execute("npm", ["remove"])
    }

    fn add(pkg: &str) -> io::Result<Output> {
        execute("npm", ["add"])
    }
}

pub struct Yarn {}

impl PackageManager for Yarn {
    fn install() -> io::Result<Output> {
        execute("yaarn", ["install"])
    }

    fn remove(pkg: &str) -> io::Result<Output> {
        execute("yaarn", ["remove"])
    }

    fn add(pkg: &str) -> io::Result<Output> {
        execute("yaarn", ["add"])
    }
}
