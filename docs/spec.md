FUNCTION OUTPUT VARIABLES:
These should be arrays.
mutliline output 

VARIABLES:
```
# a supporting multiline out
dckr_list: array = $(sudo docker ps -aq)

uname_tkn: string = "uname"

yeet: char = 'a'
```

VARIABLE LENGTH ARRAYS:
```
# definition
my_list: List<str> = ['banana', 'apple', 'fucky']

# default functionality implies to the end of the list
my_list.push(item)

# you can also push to a specific index
my_list.push(item, index)
```

FLOW LOGIC
```
# if exit code of previous command does not return 0 "not"==! exit_code=='e?' 
if [ !e? -eq 0 ] {

    # exit script with 0
    exit 0;

} elif [!e? -eq 0 || !e? -eq 1 ] {

    # exit script with 1
    exit 1;

} else {

    # some func we call
    my_favfunc();

}
```

MATCH: 
```tbsh
match opt {
  "1" -> {
    do_this()
  }

  "2" | "3" -> {
    do_what_you_are_supposed_to_do()
  }

  any -> {
    do_nothing()
  }
}
```

COMMENTS: 
```tbsh
# single line comments look like this...

### multi line comments 
    look like this ###
```
