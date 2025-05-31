# FLYUX Programming Language (JIT Version)

FLYUX is a minimalist yet powerful programming language designed for clear syntax, expressive code, and unique language features. FLYUX simplifies complex programming tasks with intuitive constructs and native support for Unicode identifiers, allowing the use of natural languages and emojis as variable names.

## Key Features
###	Minimal and Elegant Syntax
•	Clean and intuitive statements without unnecessary keywords.	
###	Native Unicode Support
•	Directly use Unicode characters, emojis, and multilingual identifiers:
```fx
🚀 := 5
输出 := 🚀 + 3
```

###	Flexible Method Call Syntax
•	Use .> for chaining function calls:
 
```fx
result := 10.>add(2).>multiply(5)
```

###	Simple Conditional Statements
•	Use if without extra keywords for multiple conditions:

```fx
if(score >= 90) { grade := "A" }
(score >= 80) { grade := "B" }
(score >= 70) { grade := "C" }
```

###	Versatile Loop Structures
•	Multiple looping styles with concise syntax (L>):
```fx
L>[10]{ print("repeat 10 times") }

L>(i := 0; i < 5; i++){ print(i) }

nums := [1, 2, 3]
L>nums:item{ print(item) }
```


###	Multiple Comparisons and Logical Expressions
•	Chain multiple comparisons elegantly:

```fx
if(0 < score <= 100 && valid){ print("valid score") }
```


## Example Programs

### Simple .> method chaining:

```fx
F>add(a, b){
    R>a+b
}

F>main(){
    result := 2.>add(3).>add(4)
    print(result) // Outputs: 9
}
```

### Concise loop and Unicode identifiers:
```fx
F>main(){
    数组 := ["🍎","🍌","🍒"]
    L>数组:item{
        print("水果:", item)
    }
}
```


## License

FLYUX is open-source software licensed under the MIT License.
