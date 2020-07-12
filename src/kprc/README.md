# Kaprino compiler (kprc)

This is a compiler for kaprino, which let you turn kaprino code into an executable file.

Kprc calls Clang internally so please **install Clang** before running kprc.

## Usage

```
kprc [file-name] (--llvm-ir) (--run) (--version)
```

- `--llvm-ir` : Emit only LLVM IR file.
- `-r` : This works as same as `--run`.
- `--run` : Launch a executable generated after compiling. If compiling was failed, this option will be ignored.
- `-v` : This works as same as `--version`.
- `--version` : Show the version of kprc.

## CMake build options

There are some options available.  
Note that a feature of kprc to optimize the output is **under constuction** currently so it's not sure that `KAPRINO_OPTIMIZER_ON` works correctly.

|Name|Type|Description|Default|
|---|---|---|---|
|KAPRINO_MONOTONE_LOG|BOOL|Make stdout monotone by disabling ANSI escape code generation|OFF|
|KAPRINO_OPTIMIZER_ON|BOOL|Optimize output (**Experimental**)|OFF|
