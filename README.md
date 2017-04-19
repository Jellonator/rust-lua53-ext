# rust-lua53-ext
Abstraction layer between Rust and Lua

This is an extension to jcmoyer's [rust-lua53](https://github.com/jcmoyer/rust-lua53) Lua bindings. It makes interfacing between Rust and Lua much simpler and more Rust-like.

Even though this is an abstraction layer, you still need to have an understanding of Lua's state system and how to use rust-lua53.

## Why use this?
When interfacing with a Lua State, you have to remember things about the Lua stack such as what the type of a given value at a position is, where each value you are working with is on the stack, how big the stack is, etc.

lua53-ext doest all of this stuff for you. Instead of working with the stack, instead you work with a series of Contexts. When you push some variables onto the stack with a context, all of the pushed values are popped when the Context goes out of scope.

## Context
A `Context` is really just a wrapper over a Lua State. A Context can be created anywhere you have a Lua State or another Context using the `Context::new` function or the `Context::push_context` function.

## Variables
When you push a value onto the lua stack with a Context, you get a value as a return that represents the pushed value's Index. For example, you can push an integer onto the stack via `Context::push_integer`. It takes a single argument, an integer, and returns a `LuaInteger`, which represents the index of the newly pushed integer (as well as some helpful abstractions).

## Example
```Rust
use context::Context;
use lua::{State, Type};
use types::LuaFunction;

fn main() {
    // Create a new rust-lua53 state
    let mut state = State::new();
    // Create a new Lua Context (Where the magic happens)
    let mut context = Context::new(&mut state);
    // Run a bit of code that creates a variable 'foo' with a value of 12.
    context.do_string("foo = 12").unwrap();
    // Push the global value 'foo' onto the stack as a LuaGeneric
    let lua_foo = context.push_global("foo");
    // Convert the LuaGeneric into an integer
    let result = lua_foo.get_value::<i64>(&mut context).unwrap();
    // Check result (12 = 12)
    assert_eq!(result, 12);
}
```

## Caveats
Lua types are independent from the context that they are created from. The following are all valid:
```Rust
fn main {
    let mut state = State::new();
    let mut context = Context::new(&mut state);
    let value;
    {
        // create a new context that sits on top of the old context
        let mut new_context = context.push_context();
        // value is now referencing a value on the stack
        value = new_context.push_integer(5);
        // the new context goes out of scope and 'value' is popped from the stack
    }
    context.push_integer(16);
    // Yet, I can still use 'value' here!
    context.set_global("foo", &value);
    // Crashes here (16 != 5)
    assert_eq!(Some(5), context.get_global("foo"));
}
```

```Rust
fn main {
    // create a context 'foo'
    let mut state_foo = State::new();
    let mut context_foo = Context::new(&mut state_foo);

    // create a context 'bar'
    let mut state_bar = State::new();
    let mut context_bar = Context::new(&mut state_bar);
    
    // create some values for 'foo' and 'bar' (both have index 1)
    let value_foo = context_foo.push_integer(1);
    let value_bar = context_bar.push_integer(2);
    
    // woops! value_bar is defined for context_bar, not context_foo!
    context_foo.set_global("baz", &value_bar);
    
    // Crash here (1 != 2)
    assert_eq!(Some(2), context_foo.get_global("baz"));
}
```
