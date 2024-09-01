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

- &str
- String
