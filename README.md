# oci-registry-as-model-registry

## Introduction

Proof of concept of a tool that allows to store models, along with
their dependencies, such as:

- Metadata
  - Number of parameters
  - Intended usage
  - Maintainers
  - ... Anything
- Tokenizer
- LoRA adapters
- ... Anything

Each layer of the OCI artifact can have its own metadata, so that it's
possible to use this information in order to combine all data in the
different layers.

Each layer can have a version embedded on the mime-type, meaning that
it's possible to be backwards-compatible from day-0.

The model itself (toplevel manifest) has its own mime-type, with
optional versioning as well.

### Help

```
❯ oci-registry-as-model-registry
Usage: oci-registry-as-model-registry <COMMAND>

Commands:
  describe
  list
  pull
  push
  run
  run-raw
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Demo

You can find a demo at the `/demo` directory.
