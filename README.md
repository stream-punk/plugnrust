plugnrust
=========

I am not affiliated with [Blue Cat Audio](https://www.bluecataudio.com), so
please report issues with this template here.

This is a rust version of the headers and reference script of [Blue Cat's Plug'n
Script](https://www.bluecataudio.com/Products/Product_PlugNScript/). I am
using this for my project, but I haven't used everything yet, so there might
still be bugs. Issues and contributions are welcome.

For documentation please consult the
[headers](https://github.com/bluecataudio/plugnscript/tree/master/NativeSource/include)
and the [reference
script](https://www.bluecataudio.com/Doc/Product_PlugNScript/#NativeReference.ScriptC).

It is not my goal to create a state of the art rust-crate, I am totally ok with
some unsafe here and there.

Linking
-------

Rust cannot create macOS shared-libraries for `dlopen` (bundles). The official way
to create bundles is to create a static-library and link it with `clang`. As I
do in the Makefile.

I will look into Windows, once my project is past alpha.
