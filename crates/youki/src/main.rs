//! # Youki
//! Container Runtime written in Rust, inspired by [railcar](https://github.com/oracle/railcar)
//! This crate provides a container runtime which can be used by a high-level container runtime to run containers.

use anyhow::Result;
use libcontainer::workload::default::DefaultExecutor;
use youki::run_youki;

fn main() -> Result<()> {
    run_youki(DefaultExecutor{})
}
