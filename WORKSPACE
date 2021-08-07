load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

### <rules-rust> ###

http_archive(
    name = "rules_rust",
    sha256 = "0fda951ff9e08ca99103f87b0b59aef07c1455a9be59b2e9d771dea785dc8a97",
    strip_prefix = "rules_rust-73f228f824b7b34549d3c7113056a40d87c17185",
    urls = [
        # `main` branch as of 2021-08-07
        "https://github.com/bazelbuild/rules_rust/archive/73f228f824b7b34549d3c7113056a40d87c17185.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(
    edition = "2018",
    include_rustc_srcs = True,
    version = "1.54.0",
)

#### </rules-rust> ###

#### <rust-analyzer> ###

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_deps")

rust_analyzer_deps()

### </rust-analyzer> ###

### <crate-universe> ###

load("@rules_rust//crate_universe:defs.bzl", "crate", "crate_universe")

crate_universe(
    name = "crates",
    packages = [
        crate.spec(
            name = "prost",
            semver = "=0.8.0",
        ),
        crate.spec(
            name = "tokio",
            semver = "=1.9.0",
            features = ["macros", "rt-multi-thread"],
        ),
        crate.spec(
            name = "tonic",
            semver = "=0.5.0",
        ),
        crate.spec(
            name = "tonic-build",
            semver = "=0.5.1",
        ),
    ],
    resolver_download_url_template = "https://github.com/bazelbuild/rules_rust/releases/download/crate_universe-13/crate_universe_resolver-{host_triple}{extension}",
    resolver_sha256s = {
        "aarch64-apple-darwin": "c6017cd8a4fee0f1796a8db184e9d64445dd340b7f48a65130d7ee61b97051b4",
        "aarch64-unknown-linux-gnu": "d0a310b03b8147e234e44f6a93e8478c260a7c330e5b35515336e7dd67150f35",
        "x86_64-apple-darwin": "762f1c77b3cf1de8e84d7471442af1314157efd90720c7e1f2fff68556830ee2",
        "x86_64-pc-windows-gnu": "c44bd97373d690587e74448b13267077d133f04e89bedfc9d521ae8ba55dddb9",
        "x86_64-unknown-linux-gnu": "aebf51af6a3dd33fdac463b35b0c3f4c47ab93e052099199673289e2025e5824",
    },
    supported_targets = [
        "x86_64-apple-darwin",
        "x86_64-unknown-linux-gnu",
    ],
    # REPIN=1 bazel build ...
    lockfile = "//:crate_universe.lock",
)

load("@crates//:defs.bzl", "pinned_rust_install")

pinned_rust_install()

#### </crate-universe> ###

#### <rules_proto> ###

http_archive(
    name = "rules_proto",
    sha256 = "602e7161d9195e50246177e7c55b2f39950a9cf7366f74ed5f22fd45750cd208",
    strip_prefix = "rules_proto-97d8af4dc474595af3900dd85cb3a29ad28cc313",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_proto/archive/97d8af4dc474595af3900dd85cb3a29ad28cc313.tar.gz",
        "https://github.com/bazelbuild/rules_proto/archive/97d8af4dc474595af3900dd85cb3a29ad28cc313.tar.gz",
    ],
)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies")

rules_proto_dependencies()

#### </rules_proto> ###
