plist2json
===

A cli to convert Apple property list files to JSON.

```
Usage: plist2json <FILENAME>

Arguments:
  <FILENAME>  Path to plist file

Options:
  -h, --help     Print help
  -V, --version  Print version
```

If you want the output pretty printed, it is recommended to pipe the output to `jq` like so.

```shell
plist2json /path/to/file.plist | jq
```
