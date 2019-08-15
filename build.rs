use cc;
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    let target = env::var("TARGET").unwrap();

    let luajit_dir = format!("{}/luajit", env!("CARGO_MANIFEST_DIR"));
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = format!("{}/luajit/src", out_dir);

    dbg!(&luajit_dir);
    dbg!(&out_dir);
    dbg!(&src_dir);

    if cfg!(target_env = "msvc") {
        let lib_path = format!("{}/lua51.lib", &src_dir);
        dbg!(&lib_path);
        if !std::fs::metadata(&lib_path).is_ok() {
            let cl_exe: cc::Tool = cc::windows_registry::find_tool(&target, "cl.exe").unwrap();
            let msvcbuild_bat = format!("{}/msvcbuild.bat", &src_dir);

            dbg!(&msvcbuild_bat);

            let mut copy_options = CopyOptions::new();
            copy_options.overwrite = true;
            dir::copy(&luajit_dir, &out_dir, &copy_options).unwrap();

            let mut buildcmd = Command::new(msvcbuild_bat);
            for (name, value) in cl_exe.env() {
                buildcmd.env(name, value);
            }
            buildcmd.env("Configuration", "Release");
            buildcmd.args(&["static"]);
            buildcmd.current_dir(&src_dir);
            buildcmd.stderr(Stdio::inherit());

            let mut child = buildcmd.spawn().expect("failed to run msvcbuild.bat");

            if !child
                .wait()
                .map(|status| status.success())
                .map_err(|_| false)
                .unwrap_or(false)
            {
                panic!("Failed to build luajit");
            }
        }
        println!("cargo:rustc-link-search=native={}", src_dir);
        println!("cargo:rustc-link-lib=static=lua51");
    } else {
        let lib_path = format!("{}/luajit.a", &src_dir);
        dbg!(&lib_path);
        if !std::fs::metadata(&lib_path).is_ok() {
            let mut copy_options = CopyOptions::new();
            copy_options.overwrite = true;
            dir::copy(&luajit_dir, &out_dir, &copy_options).unwrap();

            let mut buildcmd = Command::new("make");
            buildcmd.current_dir(&src_dir);
            buildcmd.stderr(Stdio::inherit());

            let mut child = buildcmd.spawn().expect("failed to run make");

            if !child
                .wait()
                .map(|status| status.success())
                .map_err(|_| false)
                .unwrap_or(false)
            {
                panic!("Failed to build luajit");
            }
        }
        println!("cargo:rustc-link-search=native={}", src_dir);
        println!("cargo:rustc-link-lib=static=luajit");
    }
}
