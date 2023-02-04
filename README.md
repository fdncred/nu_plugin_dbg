# Readme

This plugin is intended to be run like so:
```nu
ls | dbg | get name | dbg
```
The idea is to allow you to get debug information from each step in the pipeline. 

I was thinking about naming it `inspect`, rather than `dbg` as `dbg` can be confused with `debug`. Alternatively we could roll this functionality into `debug`.

## Sample Output

I'd really like to clean this up somehow.
![dbg output](https://raw.githubusercontent.com/fdncred/nu_plugin_dbg/main/assets/dbg2.png)

```
> ls | dbg | get name | dbg
input description: table<name: string, type: string, size: filesize, modified: date>

input value: [{name: Cargo.lock, type: file, size: 36.6 KiB, modified: Sun, 22 Jan 2023 07:46:48 -0600 (34 seconds ago)}{name: Cargo.toml, type: file, size: 521 B, modified: Sun, 22 Jan 2023 07:46:46 -0600 (36 seconds ago)}{name: LICENSE, type: file, size: 1.1 KiB, modified: Sat, 24 Dec 2022 13:48:43 -0600 (4 weeks ago)}{name: README.md, type: file, size: 1.2 KiB, modified: Sat, 24 Dec 2022 13:48:43 -0600 (4 weeks ago)}{name: src, type: dir, size: 128 B, modified: Sat, 24 Dec 2022 13:48:43 -0600 (4 weeks ago)}{name: target, type: dir, size: 192 B, modified: Fri, 30 Dec 2022 07:32:48 -0600 (3 weeks ago)}]

input description: list<string>

input value: [Cargo.lockCargo.tomlLICENSEREADME.mdsrctarget]

╭───┬────────────╮
│ 0 │ Cargo.lock │
│ 1 │ Cargo.toml │
│ 2 │ LICENSE    │
│ 3 │ README.md  │
│ 4 │ src        │
│ 5 │ target     │
╰───┴────────────╯
```