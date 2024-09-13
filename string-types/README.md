#### Safety

- No null terminator
- Immutable by default
- UTF-8 encoding.

### String

- Heap Allocated
- Ownable
- Growable
- UTF-8 encoded
- usefull when create/modify the strings
  ![String]("./String.png")
- This format helps in easier manipulation

### &str

- string slice
- view to String
- does't own the data
- not Growable
- access data in heap or in binary or in stack (rare but possible)
- Read/analyze strings
- command arguments, or searching for a substring
  ![str]("./str.png")

### Box<str>

- owned
- not Growable
- heap-allocated string size
- usefull when freeze a string to prevent furture modification. by dropping extra capicity info. in String type
- usecase in apis which return owned string which is not modified further or to save memory aggresivly to imp. prefromance
