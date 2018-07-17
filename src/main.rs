extern crate cargo;
extern crate fs_extra;
extern crate clap;

use std::path::Path;
use std::fs;

use cargo::{
    ops,
    core::Workspace,
    util::Config
};

use fs_extra::dir::{self, CopyOptions};

use clap::{App, Arg};

fn download_deps<P: AsRef<Path>, Q: AsRef<Path>>(toml: P, dest: Q) {
    let config = Config::default().unwrap();
    let workspace = Workspace::new(&fs::canonicalize(toml).unwrap(), &config).unwrap();
    let (package, _) = ops::resolve_ws(&workspace).unwrap();
    for id in package.package_ids() {
        let p = package.get(id).unwrap();
        if p.name() == workspace.current().unwrap().name() {continue;}
        dir::copy(p.root(), fs::canonicalize(&dest).unwrap(), &CopyOptions {
            overwrite: true,
            skip_exist: false,
            buffer_size: 64000,
            copy_inside: true,
            depth: 0
        }).unwrap();
    }
}

fn main() {
    let args = App::new("cargo download-deps")
       .version("1.0")
       .about("Does great things!")
       .author("Robby M.")
       .arg(Arg::with_name("TOML").required(true))
       .arg(Arg::with_name("DEST").required(true))
       .get_matches();
    let toml = args.value_of("TOML").unwrap();
    let dest = args.value_of("DEST").unwrap();
    download_deps(toml, dest);
}
