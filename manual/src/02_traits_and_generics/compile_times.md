# Compile Times

It's worth noting that heavy use of generics can make your compile times a lot longer. This is because the compiler has to generate separate code for each type you use with a generic function or struct. If you use a generic function with many different types, the compiler has to generate a lot of code, which can slow down compilation.

To mitigate this, you can try to limit the number of different types you use with a generic function or struct. You can also use trait objects (with `dyn`) in some cases to reduce the number of generated versions of a function, but this comes at the cost of some performance due to dynamic dispatch.

You can also help by avoiding exposing generics over crate boundaries, and instead provide concrete access functions/types. (Obviously, that's not always practical). That limits the compile time delay to a single crate, rather than every crate that uses your crate.