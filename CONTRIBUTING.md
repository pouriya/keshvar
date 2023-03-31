# To contributors
I ❤️ PR from everyone, and I appreciate your help but before opening a PR, make an issue and describe your feature, bug, etc.

### Generate & Build Workflow
Most parts of `keshvar` library are auto-generated. Actually each file that starts with Rust comment `// DO NOT TOUCH THIS FILE. ...` is auto-generated.  There is another Rust crate (application) that generates these source files. You can find source code of the generator inside [`keshvar-code-generator` directory](https://github.com/pouriya/keshvar/tree/master/keshvar-code-generator). The generator uses YAML data files to generated most parts of `keshvar` library. These YAML files are parts of another repository called [`countries`](https://github.com/countries/countries) which is written in Ruby. So this is how the `keshvar` library is generated:
* We have some source files of `keshvar` library (Parts that we wrote by hand).  
* We pull [`countries`](https://github.com/countries/countries) library as a Git submodule.
* We tell `keshvar-code-generator` application to use YAML files (inside [`countries`](https://github.com/countries/countries) library) and generate other parts of `keshvar` library.

### Keep in mind (before doing anything)
* We should not change to the data inside **this repository**. We need to make a PR to [`countries`](https://github.com/countries/countries) library instead.
* We are not allowed to change auto-generated files by hand. Change the generator instead.
* After re-generating source files, We should use `cargo fmt` to fix the style of newly generated source files. 
* `keshvar` and `keshvar-code-generator` are two separate Rust crates. Do not combine them.
* [**Makefile**](https://github.com/pouriya/keshvar/blob/master/Makefile) is your friend.  

