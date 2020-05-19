# Kaffee

A Rust-powered "JEsque" programming language

## What?

Kaffee is a toy project of mine to create a tokeniser, parser, lexer, interpretter
and garbage collector in Rust.

## Why?

As well as being a fun side-project as my first real Rust program, my overall
goal is to create a language similar to JavaScript, but without the parts which
frustrate and baffle many people who don't use it regularly.

For example, the type system does not include `undefined` or `NaN`, accessing
a non-existent key in an object throws, as does dividing by zero, rather than
breaking obscure parts of your code by sending these unexpected values through
it silently.

## Features

 - Objects - Object literals, computed property access, etc.
 - No hoisted variables - Variables are exclusively block-scoped
 - More predictable boolean coercion - `null` is the only value which is falsy (besides `false`)
 - Tracing garbage collector
 - Rust/Python-like `if` syntax

## Examples

**Hello world:**

```js
println("Hello, world!")
```

**Fizzbuzz:**
```js
for (let i = 1; i <= 100; i++) {
  let out = ""
  if i % 3 == 0 out = "Fizz"
  if i % 5 == 0 out += "Buzz"
  if out == "" out = i
  println(out)
}
```

Note that most of the syntax in the `for` statement is optional, to make the
language feel more comfortable. Kaffee doesn't need the brackets, semi-colons,
etc. to know what you mean.

```js
for let i = 0 i < 5 i++ println(i)
```

**Recursive factorial:**
```js
function factorial(n) {
  if n < 2 return n
  return n * factorial(n - 1)
}
println(factorial(5))
```

**Objects:**

Objects can contain functions, functions can return objects, new keys
can be created with assignment.

```js
function myFunc () {
  return {
    subFunc: function () {
      println("I'm a first-class function")
    }
  }
}

const person = {
  name: "Adam",
  myFunc
}

person.age = 17
person.myFunc().subFunc()
```
