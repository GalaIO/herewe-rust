use std::fs::File;

fn main() {
    // rust与go、c语言一样，通过errcode的方式处理错误，而不是异常
    // 这样就导致在进行函数调用时，必须手动对错误进行check和处理
    // 而不像异常可以只关注逻辑，然后在外部try-catch进行统一处理，这样很有可能让程序员忘记异常处理
    // 但是像java也是希望可以手动处理或者意识到异常产生，比如checkedException，但是实际使用较少，都是RuntimeException，懒而已。。。
    // 在rust中对null的处理已经看到过了 就是主动返回Option枚举，来让调用者判断如何处理，还有一个辅助函数比如unwrap之类
    // 在go中null就是零值，对于指针来说就是nil存在空指针风险
    // 而错误处理，rust也分为panic和Result两种，一种是不可恢复错误，一种是需要调用者check的错误，可恢复错误

    // panic不可恢复错误
    test_panic();

    // check错误，可恢复错误
    test_result();
}

fn test_result() {
    // rust中定义了内置的错误类型，go中也有比如error借口哦，但是这个功能要强大的多，因为它支持泛型也支持枚举
    // Result有两个变体，一个成功，一个失败，分别携带成功的结果和异常的结果
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // 对于可能出现错误怎么处理呢？一个就是return返回，往上抛让调用者处理，就是错误传播，传播的时候可以wrapper一层错误，来增加特殊的错误信息
    // 比如在下载文件的时候，可以写下载保存失败返回
    let result = File::open("hello.txt");
    let f = match result {
        Ok(f) => f,
        Err(e) => {
            panic!("some error: {:?}", e)
        },
    };

    // 或者不使用match同时处理两种情况，可以使用is_err判断是否错误直接处理，然后再unwrap获取结果
    if result.is_err() {
        panic!("some error: {:?}", e)
    }
    // 注意直接使用unwrap在存在错误时直接panic，所以要注意是否符合预期，是直接panic还是保证会成功下使用unwrap
    let f = result.unwrap();
    // 如果是期望panic的话，又想增加msg可以使用expect
    let f = result.expect("some error");

}

fn test_panic() {
    // 通过panic!可以出发一个不可恢复panic，同时panic会触发栈展开，意味着会沿着调用栈反向顺序遍历所有调用函数，类似java异常的堆栈信息
    // 非常有利于排查问题，比如一个数组越界问题，最终肯定是标准库panic但是从那段逻辑引起就需要堆栈，在rust中栈展开会额外存储很多信息，导致可执行文件扩大
    // 如果在不需要使用栈展开下，可以在Cargo.toml文件[profile]增加panic='abort'来直接终止程序而不是栈展开
    // 还有一点，默认栈展开只会打印当前触发panic的位置，需要增加RUST_BACKTRACE=full来打印完整的堆栈

    // 默认debug模式下的栈展开信息
    // RUST_BACKTRACE=full cargo run
    // thread 'main' panicked at 'crash and burn', learn_error/src/main.rs:14:5
    // stack backtrace:
    // 0:        0x101fd8134 - std::backtrace_rs::backtrace::libunwind::trace::hf8598643f9eebc26
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
    // 1:        0x101fd8134 - std::backtrace_rs::backtrace::trace_unsynchronized::h19a3bd07f8cb7f35
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/../../backtrace/src/backtrace/mod.rs:66
    // 2:        0x101fd8134 - std::sys_common::backtrace::_print_fmt::h3fc167841ad5be48
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/sys_common/backtrace.rs:79
    // 3:        0x101fd8134 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbb144aff6cb2709e
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/sys_common/backtrace.rs:58
    // 4:        0x101fee290 - core::fmt::write::h0ce880d33cd2a300
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/core/src/fmt/mod.rs:1080
    // 5:        0x101fd66b6 - std::io::Write::write_fmt::h2bb1b1639b8478a0
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/io/mod.rs:1516
    // 6:        0x101fd9a8f - std::sys_common::backtrace::_print::h1418657e0e107c4e
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/sys_common/backtrace.rs:61
    // 7:        0x101fd9a8f - std::sys_common::backtrace::print::h3c3eba71f67c7cbd
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/sys_common/backtrace.rs:48
    // 8:        0x101fd9a8f - std::panicking::default_hook::{{closure}}::hb543bf3b05c8104b
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/panicking.rs:208
    // 9:        0x101fd975d - std::panicking::default_hook::h7015a8c764df2477
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/panicking.rs:227
    // 10:        0x101fda05b - std::panicking::rust_panic_with_hook::hb4c39ab085f5a3ba
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/panicking.rs:577
    // 11:        0x101fc6af7 - std::panicking::begin_panic::{{closure}}::ha1c5e3447e241d38
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:506
    // 12:        0x101fc6ca8 - std::sys_common::backtrace::__rust_end_short_backtrace::h1bfb8bb2eb263763
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:153
    // 13:        0x101ff1e97 - std::panicking::begin_panic::h8709d2c607091ba1
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:505
    // 14:        0x101fc670c - learn_error::main::h8bc1f2750cb08283
    // at /Users/galaio/RustProjects/herewerust/learn_error/src/main.rs:14
    // 15:        0x101fc628e - core::ops::function::FnOnce::call_once::ha6b83bba64fd4963
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
    // 16:        0x101fc6ce1 - std::sys_common::backtrace::__rust_begin_short_backtrace::h748e1cda678638dc
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys_common/backtrace.rs:137
    // 17:        0x101fc6904 - std::rt::lang_start::{{closure}}::h9b1ec45d2d9eb5c4
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:66
    // 18:        0x101fda3d4 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h31ef09f9bee131c1
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/core/src/ops/function.rs:259
    // 19:        0x101fda3d4 - std::panicking::try::do_call::h693e4d9f7366f6d8
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/panicking.rs:381
    // 20:        0x101fda3d4 - std::panicking::try::h58e51096fb939dc8
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/panicking.rs:345
    // 21:        0x101fda3d4 - std::panic::catch_unwind::h4406f92fae32aea8
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/panic.rs:382
    // 22:        0x101fda3d4 - std::rt::lang_start_internal::h5e1feb19f4099625
    // at /rustc/a1dfd2490a6cb456b92e469fa550dc217e20ad6d/library/std/src/rt.rs:51
    // 23:        0x101fc68e1 - std::rt::lang_start::h558a3c8ac28a6857
    // at /Users/galaio/.rustup/toolchains/nightly-2020-10-06-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/rt.rs:65
    // 24:        0x101fc6732 - _main
    //
    // release模式下的栈展开信息
    // RUST_BACKTRACE=full cargo run --release
    // thread 'main' panicked at 'crash and burn', learn_error/src/main.rs:14:5
    // stack backtrace:
    // 0:        0x10dd7d344 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbb144aff6cb2709e
    // 1:        0x10dd93410 - core::fmt::write::h0ce880d33cd2a300
    // 2:        0x10dd7b8c6 - std::io::Write::write_fmt::h2bb1b1639b8478a0
    // 3:        0x10dd7ec0f - std::panicking::default_hook::{{closure}}::hb543bf3b05c8104b
    // 4:        0x10dd7e8dd - std::panicking::default_hook::h7015a8c764df2477
    // 5:        0x10dd7f1db - std::panicking::rust_panic_with_hook::hb4c39ab085f5a3ba
    // 6:        0x10dd6c1dd - std::panicking::begin_panic::{{closure}}::ha1c5e3447e241d38
    // 7:        0x10dd6c188 - std::sys_common::backtrace::__rust_end_short_backtrace::h1bfb8bb2eb263763
    // 8:        0x10dd96fee - std::panicking::begin_panic::h8709d2c607091ba1
    // 9:        0x10dd6c11c - learn_error::main::h8bc1f2750cb08283
    // 10:        0x10dd6c19a - std::sys_common::backtrace::__rust_begin_short_backtrace::h748e1cda678638dc
    // 11:        0x10dd6c1fc - std::rt::lang_start::{{closure}}::h9b1ec45d2d9eb5c4
    // 12:        0x10dd7f554 - std::rt::lang_start_internal::h5e1feb19f4099625
    // 13:        0x10dd6c149 - _main

    // 所以可以发现release模式简化了多少。。。真是为了性能
    // 错误信息多么有用，而且尽量对错误细化，非常有利于排查问题，比如根据错误码或者msg就可以定位
    // panic!("crash and burn")

    // panic使用场景要额外注意，是否符合设计预期，因为一旦panic就没有恢复的可能
    // 对于在示例、原型、测试等场景可以直接panic，这时候对错误的处理比较简单，随后当进一步增强程序健壮性时，再把不必要或者需要处理的错误使用Result返回

}
