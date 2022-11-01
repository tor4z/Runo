# Runo Unikernel

## Build

```bash
cargo xtask build
```

## Run in qemu

```bash
cargo xtask qemu
```

## TODO

- add multi-thread module
- file system module
- memory manage module
- optional multi-process support. For a pure unikernel system, single process is enough, but we want to extent the unikernel system to genernal system with a compile option.


## Create a new lib-package

```bash
cargo xtask new [NAME] --lib
```

## Create a new app-package

```bash
cargo xtask new [NAME] --app
```
