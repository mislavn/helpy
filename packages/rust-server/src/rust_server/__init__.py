from rust_server._core import hello_from_rust, rust_error, RustClass as RustClass


def hello() -> str:
    return hello_from_rust()


def error() -> str:
    return rust_error()
