# spall

Unremarkable JS framework for the benefit of my learning. Compiler is built in Rust.

## Misc notes:

An app is made up of elements, which are stored in the `elements/` directory. They're basically components but with a better name. Element file names are in format `{element name}.spall`. There must be an element called `Root` which is the root of your app.

The `meta/` dir of a project contains stuff that is not the app itself. `index.html` is the entry point into the app and is plain html. You should put stuff like linking to the renderer in there!

When an app is built, lots of files are created in the `build/` directory, which can then be used in a regular server like Apache.

The `runtime/` dir contains the stuff that runs in the browser.