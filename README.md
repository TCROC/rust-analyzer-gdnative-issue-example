# broken-example

This folder illustrates the issue.  If you open it in vs code: https://github.com/microsoft/vscode , you will see that the auto completions and comment hovers are not working.  Here is a screenshot to demonstrate:

![image](https://github.com/TCROC/rust-analyzer-gdnative-issue-example/assets/24307049/001b8cab-738b-4886-855b-ef5a202a6ee3)

Cargo.toml

```toml
[package]
name = "broken-example"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
gdnative = "0.11.3"
gdnative-bindings = "0.11.3"
```

# working-example

This folder illustrates a workaround that can be applied with the gdnative-bindings library.  If you enable the "one-class-one-file" feature:

```
gdnative-bindings = { version = "0.11.3", features = [ "one-class-one-file" ] }
```

Then the generated rust code will be broken up into multiple files instead of compressed in one file.  Now the rust-analyzer works properly:

![image](https://github.com/TCROC/rust-analyzer-gdnative-issue-example/assets/24307049/420282ec-1e9c-4cd7-8484-8120339cdc83)

Cargo.toml

```Cargo.toml
[package]
name = "working-example"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
gdnative = "0.11.3"
gdnative-bindings = { version = "0.11.3", features = [ "one-class-one-file" ] }
```

Now I do think rust-analyzer should be able to handle this.  I do think it is likely a bug on the rust-analyzer side.  Fortunately, there is a workaround for the gdnative crate.  Other crates may have issues though if they generate code and do not break the code apart into different files.

I hope you adventuring lads find this project useful in your debugging journey! :)
