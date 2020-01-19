use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    let luajit_dir = format!("{}/luajit", env!("CARGO_MANIFEST_DIR"));
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = format!("{}/luajit/src", out_dir);

    dbg!(&luajit_dir);
    dbg!(&out_dir);
    dbg!(&src_dir);

    // DEP_LUAJIT_INCLUDE
    // DEB_LUAJIT_LIB_NAME

    let lib_name = build_luajit(&luajit_dir, &out_dir, &src_dir);

    println!("cargo:lib-name={}", lib_name);
    println!("cargo:include={}", src_dir);
    println!("cargo:rustc-link-search=native={}", src_dir);
    println!("cargo:rustc-link-lib=static={}", lib_name);

    // if cfg!(target_os = "macos") && cfg!(target_pointer_width = "64") {
    //     // RUSTFLAGS='-C link-args=-pagezero_size 10000 -image_base 100000000'
    // }
}

#[cfg(target_env = "msvc")]
fn build_luajit(luajit_dir: &str, out_dir: &str, src_dir: &str) -> &'static str {
    const LIB_NAME: &'static str = "lua51";
    let lib_path = format!("{}/{}.lib", &src_dir, LIB_NAME);
    dbg!(&lib_path);
    if !std::fs::metadata(&lib_path).is_ok() {
        let target = env::var("TARGET").unwrap();
        let cl_exe: cc::Tool =
            cc::windows_registry::find_tool(&target, "cl.exe").expect("cl.exe not found");
        let msvcbuild_bat = format!("{}/msvcbuild.bat", &src_dir);
        dbg!(&msvcbuild_bat);

        let mut copy_options = CopyOptions::new();
        copy_options.overwrite = true;
        dir::copy(&luajit_dir, &out_dir, &copy_options)
            .expect("failed to copy luajit source to out dir");

        let mut buildcmd = Command::new(msvcbuild_bat);
        for (name, value) in cl_exe.env() {
            eprintln!("{:?} = {:?}", name, value);
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

    LIB_NAME
}

#[cfg(not(target_env = "msvc"))]
fn build_luajit(luajit_dir: &str, out_dir: &str, src_dir: &str) -> &'static str {
    const LIB_NAME: &'static str = "luajit";
    let lib_path = format!("{}/lib{}.a", &src_dir, LIB_NAME);
    dbg!(&lib_path);
    if !std::fs::metadata(&lib_path).is_ok() {
        let mut copy_options = CopyOptions::new();
        copy_options.overwrite = true;
        dir::copy(&luajit_dir, &out_dir, &copy_options).unwrap();
        std::fs::copy(format!("etc/Makefile"), format!("{}/Makefile", &src_dir)).unwrap();

        let mut buildcmd = Command::new("make");
        buildcmd.current_dir(&src_dir);
        buildcmd.stderr(Stdio::inherit());

        if cfg!(target_pointer_width = "32") {
            buildcmd.env("HOST_CC", "gcc -m32");
            buildcmd.arg("-e");
        }

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
    LIB_NAME
}
