load("@rules_rust//rust:rust.bzl", "rust_binary")
load("@crates//:defs.bzl", "crate")

rust_binary(
    name = "server",
    srcs = [
        "server.rs",
    ],
    deps = [
        "//src/protogen",
        crate("tokio"),
        crate("tonic"),
    ],
    visibility = ["//visibility:public"],
)

rust_binary(
    name = "client",
    srcs = [
        "client.rs",
    ],
    deps = [
        "//src/protogen",
        crate("tokio"),
        crate("tonic"),
    ],
    visibility = ["//visibility:public"],
)
