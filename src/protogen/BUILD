load("@rules_rust//rust:rust.bzl", "rust_library")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@crates//:defs.bzl", "crate")

rust_library(
    name = "protogen",
    srcs = [
        "lib.rs",
    ],
    deps = [
        ":gen",
        crate("prost"),
        crate("tonic"),
    ],
    visibility = ["//visibility:public"],
)
alias(
    name = "rustfmt",
    actual = select({
        "@rules_rust//rust/platform:osx": "@rust_darwin_x86_64//:rustfmt_bin",
        "@rules_rust//rust/platform:linux": "@rust_linux_x86_64//:rustfmt_bin",
    }),
)
cargo_build_script(
    name = "gen",
    srcs = [
        "gen.rs",
    ],
    deps = [
        crate("tonic-build"),
    ],
    build_script_env = {
        "RUSTFMT": "$(execpath :rustfmt)",
        "PROTOC": "$(execpath @com_google_protobuf//:protoc)"
    },
    data = [
        "//src/proto",
        ":rustfmt",
        "@com_google_protobuf//:protoc",
    ],
)
