# On Your Marks.. Get Set.. Rust!

**Things might break and error handling is primitive**
**Use at your own risk**

This library provides a single macro `GetSet`, which allows you to write rudimentary getters and setters for your struct. 

I wrote this because I was tired of manually writing them(). In most cases, you would not need getters and setters which are this primitive but **I** was needed them (and also "muh proc-macros").
`¯\_(ツ)_/`

If you find a bug or have a feature which seems like a good fit, feel free to contribute.

---

## Basic Usage
You can use the macro in the following manner: 

```
    #[derive(GetSet)]
	struct Foo {
        #[get(clone)]
        bar: String, 
        #[get(copy)]
        pub baz: f32,
        #[get(im_ref(&Path), mut_ref(&mut Path))]
        qux: PathBuf
        #[set]
        pub quux: Option<u64>, 
	}
```

These should effectively translate to:
```
    impl Foo {
        fn get_bar(&self) -> String {
            self.bar.clone()
        }
        pub fn get_baz(&self) -> String {
            self.baz.clone()
        }
        fn get_qux_ref(&self) -> &Path {
            self.qux.as_ref()
        }
        fn get_qux_ref_mut(&self) -> &mut Path {
            self.qux.as_mut()
        }
        pub fn set_quux(&mut self, value: Option<u64>) {
            self.quux = value
        }
    }
```
## Remarks
- Each field can only have atmost **one** `#[get(..)]` attribute and atmost **one** `#[set]` attibute. 
  So, This is fine  
    ```
        ...
            #[get(clone, im_ref(&i32), mut_ref(&mut i32))]
            corge: i32
        ...
    ```
    This is not!
    ```
        ...
            #[get(clone)]
            #[get(im_ref(&i32))]
            #[get(mut_ref(&mut i32))]
            corge: i32
        ...
    ```
- Arguments `copy`, `clone`, `im_ref` and `mut_ref` can only be present at most **once**, inside an attribute.
- Arguments `copy` and `clone` can be used inside the same attribute.
- Arguments `im_ref` and `mut_ref` also require a rust type, which is the return type of the getter.
- Argument `im_ref` simply calls the `AsRef` impl of that type. Similarily `mut_ref` calls the `AsMut` impl.
- If the Argument is either `copy` or `clone`, the name of the getter will simply be `get_<field_name>`, where `<field_name>` is replaced by the name of the field on which the attribute is present. (*See the above example*)
- Similarly Argument is `im_ref`, the name of the getter will be `get_<field_name>_ref`, and `get_<field_name>_ref_mut` in case of `mut_ref`
- Attribute `#[set]` does not take any arguments and the name of the setter generated is `set_<field_name>`.

--- 
## Funky 
There also exist a special getter argument - `funky`, which allows you to do funky stuff (*duh*).
This allows you to use any rust expression to manipulate the value, which you are getting.
```
    ...
        #[get(funky(grault_opt :: grault.ok() => Option<i32>))]
        grault: Result<i32>
        #[get(funky(garply_mut_opt :: mut garply.as_mut() => Option<&mut i32>))]
        garply: Option<f32>
    ...
```
These effectively translate to: 
```
    ...
        fn get_grault_opt(&self) -> Option<i32> {
            self.grault.ok()
        }
        fn get_garply_mut_opt(&mut self) -> Option<&mut i32> {
            self.garply.as_mut()
        }
    ...
```

You must be careful while using this, because technically you can do things like: 
```
    ...
        #[get(funky(waldo_funk :: mut { 
            let foo = *waldo; 
            let bar = (foo * 420) as f32;
            *waldo = (bar - 1.69) as i32;
            bar 
        } => f32))]
        pub waldo: i32
    ...
```
This will translate to something like 
```
    ... 
        pub fn get_waldo_funk(&mut self) -> f32 {
            let foo = *self.waldo;
            let bar = (foo * 420) as f32;
            self.waldo = (bar - 1.69) as i32;
            bar
        }
    ... 
```

- I only added this, because i find myself often needing getters which call a single method on a field and return that valu- I only added this, because i find myself often needing getters which call a single method on a field and return that resulting value (*See the example*)
- Probably do not use if you need to do more than **one** line of processing on the field value (or use it if you are lazy like me :D)











