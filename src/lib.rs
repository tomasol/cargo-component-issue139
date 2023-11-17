pub mod cargo_component {
    cargo_component_bindings::generate!();

    use bindings::Guest;

    struct Component;

    impl Guest for Component {
        /// Say hello!
        fn hello_world() -> String {
            bindings::imported();
            "Hello, World!".to_string()
        }
    }
}

pub mod wit_bindgen {
    wit_bindgen::generate!({
        world: "example",
        exports: {
            world: MyHost,
        },
    });

    struct MyHost;
    impl Guest for MyHost {
        fn hello_world() -> wit_bindgen::rt::string::String {
            imported();
            "Hello, World!".to_string()
        }
    }
}
