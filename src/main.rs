extern crate cargo;
extern crate clap;
extern crate fs_extra;

use std::fs;
use std::path::Path;

use cargo::{core::Workspace, ops, util::Config};

use fs_extra::dir::{self, CopyOptions};

use clap::{App, Arg, SubCommand, AppSettings};

fn download_deps<P: AsRef<Path>, Q: AsRef<Path>>(toml: P, dest: Q) {
    let config = Config::default().unwrap();
    let workspace = Workspace::new(&fs::canonicalize(toml).unwrap(), &config).unwrap();
    let (package, _) = ops::resolve_ws(&workspace).unwrap();
    for id in package.package_ids() {
        let p = package.get(id).unwrap();
        if p.name() == workspace.current().unwrap().name() {
            continue;
        }
        fs::create_dir_all(&dest).unwrap();
        let dest = fs::canonicalize(&dest).unwrap();
        let dest_name = &dest.join(&p.name() as &str);
        if !dest_name.exists() {
            dir::copy(
                p.root(),
                &dest,
                &CopyOptions {
                    overwrite: true,
                    skip_exist: false,
                    buffer_size: 64000,
                    copy_inside: true,
                    depth: 0,
                },
            ).unwrap();
            fs::rename(&dest.join(p.root().file_name().unwrap()), &dest_name).unwrap();
        }
    }
}

fn main() {
    let args = App::new("cargo-download-deps")
        .version("1.0")
        .bin_name("cargo")
        .about("Does great things!")
        .author("Robby M.")
        .setting(AppSettings::TrailingVarArg)
        .subcommand(
            SubCommand::with_name("download-deps")
                .arg(Arg::with_name("TOML").required(true).long("config").takes_value(true))
                .arg(Arg::with_name("DEST").required(true).long("download-path").takes_value(true)),
        )
        .get_matches();
    let args = args.subcommand_matches("download-deps").unwrap();
    let toml = args.value_of("TOML").unwrap();
    let dest = args.value_of("DEST").unwrap();
    download_deps(toml, dest);
}
