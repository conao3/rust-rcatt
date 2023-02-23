# rcatt

## Implementations

### Theoretical value

```sh
$ yes | pv -F '  %a'  > /dev/null
  [5.14GiB/s]
```


### baseline (GNU cat)

```
$ cat --version
cat (GNU coreutils) 9.1
Copyright (C) 2022 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Torbjorn Granlund and Richard M. Stallman.
```

```sh
$ yes | cat | pv -F '  %a'  > /dev/null
  [2.78GiB/s]
```


### io::copy

```sh
$ yes | cargo run --example io_copy --release | pv -F '  %a'  > /dev/null
  [4.52GiB/s]
```
