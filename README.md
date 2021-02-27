
# TBSH - Typed Bash Shell

## The idea:
    The main idea for this project is to have some sort of strict type checking in bash.
    This would mean that all of your bash functions/commands/oneliners would still work mostly as intended the difference being that
    when you create a script that is marked for tbsh when executed like you would execute any other script `./script.tbsh` you start
    you instead have the tbsh scripting language instead of normal bash. This is more aimed at being mostly different to bash
    but not entirely removed.

## Where is this in development?

    This is really early in development I do not work on this really often. 
    Right now we are currently writing the lexing and parsing process for the lanauage

## Simple expression coming out of the lexer.
```
[src\main.rs:36] res = Ok(
    [
        Int(
            1,
        ),
        Whitespace,
        Plus(
            '+',
        ),
        Whitespace,
        Int(
            1,
        ),
    ],
)
```

