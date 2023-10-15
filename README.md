## `reddot` | Find executables in `$PATH` and serialize them to `JSON` list
A tiny rust tool meant to be called from within scripts and widgets.

#### Usage:
```shell
reddot --pattern <PATTERN> --exe-count <EXE_COUNT>
```

#### Options:
- `-p`, `--pattern <PATTERN>`      Pattern by which the executables will be filtered
- `-e`, `--exe-count <EXE_COUNT>`  Maximum count of executables in output
- `-h`, `--help`                   Print help

## Nix
The repository contains a `flake.nix`, so `reddot` can be easily added to `inputs` of another flake:

```nix
reddot.url = "github:mxxntype/reddot";
```

And then added to the package list like this:

```nix
home.packages = with pkgs; [
  inputs.reddot.packages.${pkgs.system}.default
];
```
