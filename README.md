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
Run a command (program or script)

```
echo "hello" | dockron program arg1 arg2 arg3
```
Run a command and pass along `stdin` and arguments

