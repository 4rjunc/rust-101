# few more concepts

texts are encoded using UTF-8

ASCII - can represent one character in one byte.
UTF-8 - one character can range from one byte to 4 bytes.

first byte represent number of characters

0xxxxxxx
110xxxxx 10xxxxxx
1110xxxx 10xxxxxx 10xxxxxx
11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

types of strings

- &str - can be in binary, stack, heap. size is dynamic. string slice. use in case you need a immutable view of string.
- String - std library, stored in heap. use in case you want to own the data and maniplulate it.
