load("@rules_rust//rust:defs.bzl", "rust_analyzer", "rust_clippy")

rust_analyzer(
    name = "rust_analyzer",
    targets = [
        "//src/protogen",
        "//src:client",
        "//src:server",
    ],
)

rust_clippy(
    name = "clippy",
    testonly = True,
    deps = [
        "//src/protogen",
        "//src:client",
        "//src:server",
    ],
)
