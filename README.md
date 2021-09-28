# Usage
```
dockron 
```
Check for `dockron` `.dockron` or `example.dockron` file in current directory. Having multiple `dockron` files will cause an error.

```
dockron program
```
```
dockron ./script.sh
```
Run a command (program or script).
#
```
echo "hello" | dockron program arg1 arg2 arg3 > file
```
Run the command `program` and pass along `stdin` and arguments, then write resulting `stdout` from `program` to file `file`.
#
```
-n <amount>
```
Run program `amount` times.

Defaults to `-n 1` to run program once.

Set to a negative value `-n -1` to run program over and over.

`-n 0` will do nothing, as the program will not be run.
#
```
-d <milliseconds>
```
Wait `milliseconds` between each run, except for the first run which starts immediately.

Defaults to `-d 0` to rerun immediately if `-n` is greater than `1`.
#
```
-v
```
Enable verbose mode.

Prints helpful debug information to `stderr`.