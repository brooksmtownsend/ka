# ka-cli

_Pronounced "cackly", like a witch laugh_

Easily kill a bunch of stuff! `ka` lists the running processes on a system, attempts to either match the process name to the argument provided or see if the executable path matches the argument.

:caution: I created this CLI to easily kill a bunch of things running locally that match a substring in the exe path so I could clean things up while testing. It is brute force, and you should use the `--dry-run` flag to make sure you aren't killing things you aren't supposed to. Proceed with caution.

## Usage
```
ka <..name> 
```

## Example
Kill all NATS servers that are running
```
ka nats-server
```

Kill all processes running that match a few process names
```
ka nats wash wasmcloud
```
