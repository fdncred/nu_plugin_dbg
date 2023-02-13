# Readme

--------------
This plugin is now a part of nushell as the `inspect` command as of 2023-02-12 https://github.com/nushell/nushell/pull/8028
--------------

This plugin is intended to be run like so:
```nu
ls | dbg | get name | dbg
```
The idea is to allow you to get debug information from each step in the pipeline. 

I was thinking about naming it something other than `dbg` as `dbg` can be confused with `debug`. Suggestions are welcome. Alternatively we could roll this functionality into `debug`.

## Sample Output

I'd really like to clean this up somehow.
![dbg output](https://raw.githubusercontent.com/fdncred/nu_plugin_dbg/main/assets/dbg2.png)

```shell
> ls | dbg | get name | dbg 
╭─────────────┬───────────────────────────────────────────────────────────────────╮
│ description │ table<name: string, type: string, size: filesize, modified: date> │
├─────────────┴──┬─────────┬─────────────┬────────────────────────────────────────┤
│ name           │ type    │ size        │ modified                               │
├────────────────┼─────────┼─────────────┼────────────────────────────────────────┤
│ Cargo.lock     │ file    │ 39.6 KiB    │ 2023-02-04T07:28:46.941580628-06:00    │
│ Cargo.toml     │ file    │ 589 B       │ 2023-02-04T07:28:46.941857876-06:00    │
│ LICENSE        │ file    │ 1.1 KiB     │ 2022-12-24T13:48:43.872126742-06:00    │
│ README.md      │ file    │ 1.5 KiB     │ 2023-01-22T07:57:05.501237065-06:00    │
│ assets         │ dir     │ 96 B        │ 2023-01-22T07:52:43.762056555-06:00    │
│ src            │ dir     │ 128 B       │ 2023-02-04T07:28:46.942071125-06:00    │
│ target         │ dir     │ 192 B       │ 2022-12-30T07:32:48.785236198-06:00    │
╰────────────────┴─────────┴─────────────┴────────────────────────────────────────╯

╭─────────────┬──────────────╮
│ description │ list<string> │
├─────────────┴──────────────┤
│                            │
├────────────────────────────┤
│ Cargo.lock                 │
│ Cargo.toml                 │
│ LICENSE                    │
│ README.md                  │
│ assets                     │
│ src                        │
│ target                     │
╰────────────────────────────╯

╭───┬────────────╮
│ 0 │ Cargo.lock │
│ 1 │ Cargo.toml │
│ 2 │ LICENSE    │
│ 3 │ README.md  │
│ 4 │ assets     │
│ 5 │ src        │
│ 6 │ target     │
╰───┴────────────╯
```
