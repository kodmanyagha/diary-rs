pub fn format_time(seconds: u32) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    pub trait Foo {
        fn implement_foo_1(&self);
    }

    pub trait Bar {
        fn implement_bar_1(&self);
    }

    pub trait FooBar: Foo + Bar {
        fn implement_foobar_1(&self) {
            self.implement_foo_1();
        }

        fn implement_foobar_2(&self);
    }

    pub struct ExampleStruct {
        pub prop1: String,
        pub prop2: i32,
    }

    /* You have to implement all structs separately. After that you can implement
    combined trait. Why? Because you can pass this struct for a specific trait. */
    impl Foo for ExampleStruct {
        fn implement_foo_1(&self) {
            //
        }
    }

    impl Bar for ExampleStruct {
        fn implement_bar_1(&self) {
            //
        }
    }

    impl FooBar for ExampleStruct {
        fn implement_foobar_2(&self) {
            //
            self.implement_foo_1();
            self.implement_bar_1();
        }
    }
}
