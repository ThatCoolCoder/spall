# Spall - A chip off the old block

> (noun) A chip or splinter of stone

Unremarkable JS framework for the benefit of my learning, is probably very similar to some other frameworks. I've build this before using a JS framework so that I'm not influenced by the design of others, although I have used C# Blazor extensively. Compiler is built in Rust. Very WIP.

## Misc notes:

An app is made up of elements, which are stored in the `elements/` directory. They're basically components but with a better name. Element file names are in format `{element name}.spall`. There must be an element called `Root` which is the root of your app. Calling them elements is somewhat confusing because HTML elements also exist. Maybe I should change the name.

The `meta/` dir of a project contains stuff that is not the app itself. `index.html` is the entry point into the app and is plain html. You should put stuff like linking to the renderer in there.

When an app is built, lots of files are created in the `build/` directory, which can then be used in a regular server like Apache.

The `runtime/` dir contains the stuff that runs in the browser, currently a single JS file. I should organise it better and compile the separate files into the bundle.