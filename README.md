Cargo-based project for attempting Bazelification on.

Created using https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

## dependency pinning

```
REPIN=1 bazel build ...
```

## rust-analyzer

Update targets for `//:rust_analyzer` if needed.

```
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```

## rustfmt

```
bazel run @rules_rust//:rustfmt
```

## clippy

```
bazel test //:clippy
```
