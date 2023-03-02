Sample based on [Deislabs - Spiderlightning](https://github.com/deislabs/spiderlightning)

# Overview
This application serves as an inferencing server for the AKri ONVIF broker and USB camera broker. It is used in Akri's end to end demo. The http server, will run in port 3000 and receive the frames as base64 strings, will process each frame using an ONNX model and return a JSON with a bounding box and confidence leve. The http serve implementation is based on [Slight http server](https://github.com/deislabs/spiderlightning) implementation, and [wasi-nn-onnx](https://github.com/deislabs/wasi-nn-onnx) for the inferencing.

# Limitations
Both Slight and WASI-nn are still in early development phases and this sample should not be modified. This means, that this sample code is strictly bounded to the current versions of **containerd-wasm-shim** (v0.4.0), **slight** (v0.3.2) and **wasi-nn-onnx**. 

# Dependencies
1. Install Ubuntu 20.04 or 22.04
1. Install [Rust Programming Language](https://www.rust-lang.org/tools/install)
1. Install [wasmtime](https://wasmtime.dev/)
1. Install [WASI](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-documents.md).
1. Install Slight -> [deislabs/spiderlightning](https://github.com/deislabs/spiderlightning).

# Running
1. From the **http-server** root folder, create the wasm binary using Rust + WASI compiler. This will create a .wasm workload under `target -> wasm32-wasi -> debug/release -> http-server-demo.wasm`
    ```
    cargo build --target wasm32-wasi
    ```

    If you want to compile a release version, use the `--release` flag
    ```
    cargo build --target wasm32-wasi --release
    ```

1. Finally, run the WASM workload using Slight
    ```
    slight -c slightfile.toml run target/wasm32-wasi/debug/http-server-demo.wasm
    ```

    If everything is running, you should be able to see the http server running on port 3000
    ```
    Server is running on port 3000
    ```

# Building a container
This sample contains a `Dockerfile` you can use for creating a container using the docker build command. You can build and push the WASM workload as any other OCI compiant container. 