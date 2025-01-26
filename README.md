# Cedar failing to create entity in android bindings

Calling `Entity::new` always returns `Err(cedar_policy::EvaluationError::RecursionLimit)` in the android bindings if there are entity attributes.

```rs
let test_entity = "Test::\"abc\"".parse().expect("parse resource uid");
let test_entity_attrs = HashMap::from([
    (
        "item1".to_string(),
        RestrictedExpression::new_string("abc".to_string()),
    ),
    ("attr2".to_string(), RestrictedExpression::new_long(123)),
]);

// This call to Entity::new is what fails in the android binding
let test_entity =
    Entity::new(test_entity, test_entity_attrs, HashSet::new()).expect("build entity");
```

Here is what the error looks like in Android Studio's Logcat:

```
FATAL EXCEPTION: main
Process: com.example.exampleapp, PID: 15076
java.lang.RuntimeException: java.lang.reflect.InvocationTargetException
	at com.android.internal.os.RuntimeInit$MethodAndArgsCaller.run(RuntimeInit.java:590)
	at com.android.internal.os.ZygoteInit.main(ZygoteInit.java:886)
Caused by: java.lang.reflect.InvocationTargetException
	at java.lang.reflect.Method.invoke(Native Method)
	at com.android.internal.os.RuntimeInit$MethodAndArgsCaller.run(RuntimeInit.java:580)
	at com.android.internal.os.ZygoteInit.main(ZygoteInit.java:886) 
Caused by: uniffi.android_bindings.InternalException: build entity: EntityAttrEvaluationError { uid: EntityUid(EntityUID { ty: EntityType(Name(InternalName { id: Id("Test"), path: [], loc: Some(Loc { span: SourceSpan { offset: SourceOffset(0), length: 4 }, src: "Test::\"abc\"" }) })), eid: Eid("abc"), loc: Some(Loc { span: SourceSpan { offset: SourceOffset(0), length: 11 }, src: "Test::\"abc\"" }) }), attr_or_tag: "item1", was_attr: true, err: RecursionLimit(RecursionLimitError { source_loc: None }) }
```

As we can see, the error is because of `err: RecustionLimit(RecursionLimitError { source_loc: None })`.

It seems to be caused by a call to `stacker::remaining_stack`. `stacker::remaining_stack` function works as expected when compiled returns Some(value). However, when the function is compiled to Kotlin using Uniffi, it returns `None` when called in the Kotlin code.

This call to `stacker::remaining_stack` happens in `cedar-policy-core/src/evaluator.rs` around ine 941 in the `stack_size_check` function:

```rs
#[inline(always)]
fn stack_size_check() -> Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if stacker::remaining_stack().unwrap_or(0) < REQUIRED_STACK_SPACE {
            return Err(EvaluationError::recursion_limit(None));
        }
    }
    Ok(())
}
```


## Building and running the example

1. **Core Libray Code**: The Rust library code is located in the `./corelib/src/`, while the Android binding code is located in the `./android_binding/src` directory.
2. **Compiled Bindings Code**: The Kotlin code that uses the compiled bindings can be found in `./bindings/android/exampleapp/app/src/main/jniLibst` and `./bindings/android/exampleapp/app/src/main/java/com/example/rust_android`.
3. **Kotlin Code**: The Kotlin code that uses the compiled bindings can be found in the `./bindings/android/exampleapp/app/src/main/java/com/example/exampleapp/MainActivity.kt`.

### Prerequisites for Building

1. Install up `cargo-ndk` for cross-compiling:
```
cargo install cargo-ndk
```

2. Add targets for Android:
```
rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        i686-linux-android \
        x86_64-linux-android
```

### Building the Android Binding

1. Build the rust code

```sh
cargo build
```

2. Use `cargo-ndk` to cross-compile the Rust library for Android

```sh
cargo ndk -o ./bindings/android/exampleapp/app/src/main/jniLibs --manifest-path ./Cargo.toml -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build
```

3. Generate the Uniffi bindigs for Kotlin

```sh
cargo run --bin uniffi-bindgen generate --library ./target/debug/libandroid_bindings.so --language kotlin --out-dir ./bindings/android/exampleapp/app/src/main/java/com/example/rust_android
```

4. Open the `./bindings/android/exampleapp/` directory in Android Studio and run the project.

Alternatively, run `./build.sh` if you don't want to type all of that.
