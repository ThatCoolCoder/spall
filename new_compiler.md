## Plan for the new project compiler

#### Introduction

The aim is to create a compiler that is more flexible, capable and modifiable than the existing one. The current compiler has a number of inherent issues that prevent certain features being added to Spall:
- It is too centered around the .spall files - originally these were the only type. This makes it difficult to process other types of files in the 
- It has no real indexing system for what files to compile. This makes it near-impossible to provide compile-time reference checks, a proper namespacing system, or multi-threaded compilation.
- It is not very modular. Currently a lot of the compiling functions are called from a single main function, due to scoping. Again, this was fine when .spall files were the only ones but now it's too messy. A new compiler would not have complete dynamic modularisation, but would just separate different tasks more effectively.
- It simply has too much baggage from the prototyping phase of development.

Note that changes in the project compiler will not immediately require changes in the various file compilers - only facilitate them. This is due to good separation of concerns and SRP.

#### Planning

Below is a plan for the highest level structure of the compiler
```
housekeeping - determining project directory, performing basic checks that it is a project
indexing files into a data structure containing several trees for the variou types of files. This involves only reading all the files in the relevant directories

# modules (each is a single function, optionally in a different module):
initialize build directory
perform all work on the .spall files
perform all work on the scoped css files
manage static files
```

A more rich error structure such as the one below will be needed, but this can be added after initial running. This representation only shows the structure of the errors - the information that they carry is not specified, and should be able to mostly be carried over from the existing hierarchy.
```
ProjectCompilationError
    NoElementDirectory
    NoMetaDirectory
    ... (other similar project issues)
    SpallCompilationError
        MarkupSyntaxError
            (below here the hiarchy is as usual)
        UnknownFileReference
        NoPageRoutes
        ...
    ScopedCssError
        NoMatchingSpall (when a .spall file cannot be found for the same name)
        CssSyntaxError
            UnexpectedToken
            UnexpectedEndOfFile
```

Recoding this would be a somewhat major task and so would be preferable to do in the following steps:
1. Create a new file/directory - it would not be practical to convert from the existing compiler.
2. Write the high-level function with stubs. Put logging in the stubs.
3. Write the non-module functions fully and correctly.
4. Write the module functions with as little effort as possible, to equal the capabilities of the existing compiler.
5. Start fixing up the module functions, adding the extra checks and features that the new compiler is for. This will in turn require redoing the error system.
