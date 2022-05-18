a simple cli tool which watches on output of one command overtime, on differences it will run another command

### example usage

watching on changes of a specific website each minute and send a notification on change

```sh
diff-checker  -d 'notify-send hello world' -c 'curl https://www.time.ir' -s "1m"
```

### help

```
diff-checker 0.1.0
a simple cli tool which watches on output of one command overtime, on differences it will run
another comamnd

USAGE:
    diff-checker [OPTIONS] --on-diff <ON_DIFF> --command <COMMAND>

OPTIONS:
    -c, --command <COMMAND>    command to run
    -d, --on-diff <ON_DIFF>    command to run on difference detected
    -h, --help                 Print help information
    -r, --remain               wether will continue or close after change detected
    -s, --sleep <SLEEP>        sleep between running the commands (time between running two commands
                               ) [default: 10s]
        --shell <SHELL>        shell for commands to execute shell to run commands ( default to
                               $SHELL or sh )
    -V, --version              Print version information
```
