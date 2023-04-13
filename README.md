# Simple Forkserver Fuzzer with AsanBacktraceObserver (casr feature)

This is a simple example fuzzer based on [LibAFL](https://github.com/AFLplusplus/LibAFL) to fuzz a executable instrumented by afl-cc.
It uses `AsanBacktraceObserver` with [CASR](https://github.com/ispras/casr) for crash deduplication.

## Usage
You can build this example by `cargo build --release`.
This downloads AFLplusplus/AFLplusplus, [xlnt](https://github.com/tfussell/xlnt) and compiles the example harness program in src/harness.cc with afl-cc.

## Run
After you build it you can run
`cp ./target/release/forkserver_simple_xlnt_casr .` to copy the fuzzer into this directory,
and you can run
`taskset -c 1 ./forkserver_simple_xlnt_casr ./target/release/harness ./corpus/ -t 1000` to run the fuzzer.
`taskset` binds this process to a specific core to improve the throughput.
