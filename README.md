implementing bcd(Binary-coded decimal) format

## Format spec
There is two format depends on how many digits the number has.

### compression format
If number has digits less than 15, this format is applied.

```
 <----------------- 64bit --------------->
|        |        |        |     |        |
| header | 1digit | 1digit | ... | 1digit |
|        |        |        |     |        |
    |        |        |              |
   4bit     4bit     4bit           4bit
```

### extended format
If number has digits more than 15, this format is applied.

```
 <----------------- 64bit ---------------> 
|        |                                |
| header |        heap pointer            |
|        |                                |
    |                  |              
   4bit              60bit           
```

### 手順
- [ ] compression formatの実装
  - [ ] string -> BCD のnewの実装
  - [ ] 各種演算の実装
  * ひとまず15桁以上はErrorに
- [ ] extended formatの実装
