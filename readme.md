# Zach-Calc

Zach-Calc is a pet project for me to try and better understand pattern matching, optimization, IRs,
and the likes.  

./libs/\* contains libraries to do things like create IR, interpret IR, optimize IR, and if I have enough time
on my hand, lower it to machine code. ./exes/\* contains frontends that do things such as parsing the
"calculator language", and parsing the domain specific optimization language.  
The main Zach-Calc language will be a simple, pure toy language for doing integer mathematics, It
will support things like defining and automatically memoizing functions. The Domain-Specific Zach-Opt language will 
be a declarative lisp-like language that is compiled to a bytecode that the Zach-Calc compiler then uses to optimize IR. 
I expect the optimization to be the major focus of the project as I learn how optimizations work and how basic language features are built.  

After the project is more fleshed out I might write more comprehensive documentation, and possibly
blogs about how the project works.
