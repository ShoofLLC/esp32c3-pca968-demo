# Demo project with the PCA968 driver
This project was created as demo.

## Prerequisites
Make sure to install the [esp32-c3-toolchain](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/get-started/linux-macos-setup.html) and ldproxy:
```bash
cargo install ldproxy
```

## Building
In order to build this project you must have the [PCA968 driver](https://github.com/ShoofLLC/esp32c3-pca968.git) checked out in the root folder as this project.

The reason for that is that the dependency has been defined as
```
esp32c3-pca968 = { version = "0.0.0", path = "../esp32c3-pca968" }
```
in the [Cargo.toml](Cargo.toml) file.
