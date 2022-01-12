load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crates//:defs.bzl", "crate")

rust_binary(
    name = "main",
    srcs = [
        "main.rs",
    ],
    deps = [
        crate("value-bag"),
    ],
    visibility = ["//visibility:public"],
)

