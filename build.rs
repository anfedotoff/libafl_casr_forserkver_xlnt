use std::{
    env,
    path::Path,
    process::{exit, Command, Stdio},
};

const AFL_URL: &str = "https://github.com/AFLplusplus/AFLplusplus";
const XLNT_URL: &str = "https://github.com/tfussell/xlnt";

fn main() {
    if cfg!(windows) {
        println!("cargo:warning=No support for windows yet.");
        exit(0);
    }

    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();

    let afl = format!("{}/AFLplusplus", &cwd);
    let afl_cc = format!("{}/AFLplusplus/afl-clang-fast++", &cwd);

    let afl_path = Path::new(&afl);
    let afl_cc_path = Path::new(&afl_cc);

    let xlnt = format!("{}/xlnt", &cwd);
    let xlnt_path = Path::new(&xlnt);
    let xlnt_build = format!("{}/build", &xlnt);
    let xlnt_build_path = Path::new(&xlnt_build);
    if !afl_path.exists() {
        Command::new("git")
            .arg("clone")
            .arg(AFL_URL)
            .status()
            .unwrap();
    }

    if !afl_cc_path.exists() {
        let mut afl_cc_make = Command::new("make");
        afl_cc_make.arg("all").current_dir(afl_path);
        if let Ok(llvm_config) = env::var("LLVM_CONFIG") {
            if !llvm_config.is_empty() {
                afl_cc_make.env("LLVM_CONFIG", llvm_config);
            }
        }
        afl_cc_make.status().unwrap();
    }

    if !xlnt_path.exists() {
        Command::new("git")
            .arg("clone")
            .arg(XLNT_URL)
            .status()
            .unwrap();
        Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .current_dir(xlnt_path)
            .status()
            .unwrap();
    }

    if !xlnt_build_path.exists() {
        std::fs::create_dir_all(xlnt_build_path).unwrap();
        let mut cmake = Command::new("cmake");
        cmake.args(["-D", "STATIC=ON", "-D", "TESTS=OFF"])
            .arg(&format!("-DCMAKE_CXX_COMPILER={afl_cc}"))
            .arg(&format!("-DCMAKE_CXX_FLAGS=-fsanitize=address"))
            .arg("..")
            .current_dir(xlnt_build_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        cmake.status().unwrap();
        let mut compile_command = Command::new("cmake");
        compile_command
            .args(["--build", "."])
            .env("CMAKE_BUILD_PARALLEL_LEVEL", "12")
            .current_dir(xlnt_build_path);
        compile_command.status().unwrap();
    }


    let mut compile_command = Command::new(&afl_cc);
    compile_command
        .args(["-fsanitize=address", "-Ixlnt/include", "-Ixlnt/third-party/libstudxml", "-O2", "src/harness.cc",  "xlnt/build/source/libxlnt.a", "-o"])
        .arg(&format!("{}/target/release/harness", &cwd));

    compile_command.status().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
}
