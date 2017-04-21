#![cfg(test)]
use context::Context;
use lua::{State, Type};
use types::LuaFunction;

#[test]
fn test_thread() {
    use std::thread;
    let mut state = State::new();
    state.open_libs();
    let mut context = Context::new(&mut state);

    // Create a table with a bunch of random numbers
    // Create a median function that finds the median value of the table
    // and a mean function that finds the mean of all the values
    context.do_string("
        t = {}
        math.randomseed(os.time())
        for i = 1, 10000 do
            local v = math.random()
            table.insert(t, v)
        end

        function mean()
            local ret = 0
            print(\"Start mean\")
            for i, v in ipairs(t) do
                ret = ret + v
            end
            ret = ret / #t
            print(\"End mean\")
            return ret
        end
        function median()
            print(\"Start median\")
            local ls = {}
            for i, v in ipairs(t) do
                ls[i] = v
            end
            table.sort(ls)
            local ret = ls[#ls // 2]
            print(\"End median\")
            return ret
        end
    ").unwrap();


    let luathread1 = context.push_thread();
    let luathread2 = context.push_thread();
    {
        let mut state1 = luathread1.as_state(&mut context);
        let mut state2 = luathread2.as_state(&mut context);
        let thread1 = thread::spawn(move || {
            let mut context1 = Context::new(&mut state1);
            let func = context1.push_global("mean")
                .get_value::<LuaFunction>(&mut context1).unwrap();
            let result = func.call_singleret(&mut context1, &[]).unwrap()
                .get_value::<f64>(&mut context1);
            return result.unwrap();
        });
        let thread2 = thread::spawn(move || {
            let mut context2 = Context::new(&mut state2);
            let func = context2.push_global("median")
                .get_value::<LuaFunction>(&mut context2).unwrap();
            let result = func.call_singleret(&mut context2, &[]).unwrap()
                .get_value::<f64>(&mut context2);
            return result.unwrap();
        });

        println!("Mean: {:?}", thread1.join().unwrap());
        println!("Median: {:?}", thread2.join().unwrap());
    }
}

#[test]
fn test_global() {
    let mut state = State::new();
    let mut context = Context::new(&mut state);
    context.set_global("foo", &12);
    assert_eq!(Some(12), context.push_global("foo").get_value(&mut context));
}

#[test]
fn test_table() {
    let mut state = State::new();
    let mut context = Context::new(&mut state);

    let table = context.push_table();
    table.set(&mut context, &"foo", &1.0);

    assert_eq!(Some(1.0), table.get_typed(&mut context, &"foo"));
}

#[test]
fn test_generic() {
    let mut state = State::new();
    let mut context = Context::new(&mut state);
    context.do_string("foo = 1.5").unwrap();
    let value = context.push_global("foo");

    assert_eq!(value.type_of(&mut context), Type::Number);
    assert_eq!(Some(1.5), value.get_value(&mut context));
}

#[test]
fn test_function_call() {
    let mut state = State::new();
    let mut context = Context::new(&mut state);
    context.do_string("function foo(a, b) return a + b; end").unwrap();

    let value:Option<f64> = context.push_global("foo")
        .get_value::<LuaFunction>(&mut context)
        .map(|func| func.call_singleret(&mut context, &[&1.2, &1.2]).unwrap())
        .and_then(|val|val.get_value::<f64>(&mut context));
    // let value = &func.call(&mut context, &[&1.2, &1.2], 1)[0];
    assert_eq!(value, Some(2.4));
}

#[test]
fn test_context_block() {
    let mut state = State::new();
    let mut context = Context::new(&mut state);
    let top = context.get_state().get_top();

    {
        // create a new context with pushed values
        let mut new_context = context.push_context();
        new_context.push_table();
        new_context.push_number(1.0);
        new_context.push_string("wow");
        // values are popped at end of block
    }

    assert_eq!(top, context.get_state().get_top());
}

#[test]
fn test_userdata() {
    struct Foo {
        value: i64
    }
    impl Foo {
        pub fn new() -> Foo {
            Foo {
                value: 0
            }
        }
        pub fn set(&mut self, v: i64) {
            self.value = v;
        }
        pub fn get(&self) -> i64 {
            self.value
        }
    }

    let mut state = State::new();
    let mut context = Context::new(&mut state);
    {
        let mut new_context = context.push_context();
        let udata = {
            let mut foo = Foo::new();
            foo.set(100);
            new_context.push_userdata(foo)
        };
        {
            let foo: &mut Foo = unsafe{udata.get_value(&mut new_context).unwrap()};
            assert_eq!(foo.get(), 100);
        }
    }
}

#[test]
fn test_stack_as_variable() {
    let mut state = State::new();
    let mut context = Context::new(&mut state);

    let b = context.push_integer(15);
    context.set_global("foo", &b);

    assert_eq!(context.push_global("foo").get_value(&mut context), Some(15));
}
