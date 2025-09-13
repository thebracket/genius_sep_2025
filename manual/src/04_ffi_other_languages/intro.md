# FFI: Other Languages - C

Like most languages, Rust can interoperate with C code. C is the "lingua franca" of programming languages, and many languages provide a way to call C code. The PyO3 system is using FFI under the hood - but hides some of the details.

Unlike a lot of other languages (C#, Java, Go, etc.), Rust is very close to the C memory model. A design goal was for relatively seamless interoperability with C code --- with no marshalling, delays or pauses when calling from one to the other.