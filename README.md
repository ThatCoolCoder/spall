# Spall - A chip off the old block

> Spall: (noun) A chip or splinter of stone

Unremarkable SPA JS framework for the benefit of my learning, is probably very similar to some other frameworks. I've built this before using a JS framework so that I'm not influenced by the design of others. I have used C# Blazor extensively in the past. Compiler is built in Rust. Very WIP. In this initial version everything is fully custom - custom markup language, custom tokeniser, custom build/bundle tool.

See misc notes to see what this is currently capable of.

## Misc notes:

See `demos/basic` for a basic look at how to make stuff work.

An app is made up of elements, which are stored in the `elements/` directory. They're basically components but with a better name. Element file names are in format `{element name}.spall`. There must be an element called `Root` which is the root of your app. Calling them elements is somewhat confusing because HTML elements also exist. Maybe I should change the name.

The `pages/` dir of a project holds pages. Pages are just elements that correspond to a "route". A page's route is specified inside a `<pageroute>` tag. You can specify a page title using `<title>` tags but it doesn't do anything yet.

The `meta/` dir of a project contains stuff that is not the app itself. `index.html` is the entry point into the app and is plain html. You should put stuff like linking to the renderer in there.

The `static/` dir of a project holds static files that can be accessed in the built app from `static/`.

The `common/` dir of a project holds Javascript files that can be accessed from elements and pages. Use it for shared functions or business logic - anything not directly tied to the frontend.

When an app is built, files are created in the `build/` directory, which can then be used in a regular server like Apache.

The `runtime/` dir contains the stuff that runs in the browser. It contains multiple files which are written . `build.rs` makes the project rebuild if these are changed.

#### .spall markup format

This will change a lot in the future but this should be correct for some time:

It's pretty similar to HTML and uses the same element names. It aims to be as similar to HTML/JS as possible, reusing familiar symbols/concepts.

To interpolate/template values just do it like Javascript template literals: `<p>The value is ${Math.random()}</p>`.

You can do conditionals and loops like this:
```
~if (Math.random() > 0.5) {~
    <p>It's a big number</p>
~} else {~
    <p>It's a small number</p>
~}~
```
Note that a closing tilde is optional if the Javascript ends at the end of a line. You can also run arbitrary JS code mid-render by writing something other than a conditional inside the tildes. This is useful for calculating intermediate values in complex calculations. The context of this code is inside a method of the generated class.

To add functions and state, add a script tag. Treat the script tag like an ES5 class wrapper, contents should look like:
```javascript
<script>

    onInitialized() {
        // code to be called when element is first created, before first render
        this.value = 100;
    }

    onRender() {
        // code to be called after every render
    }

    someCustomFunction() {
        return 42;
    }
</script>
```

Of course, attributes like `style` work on tags.

Callbacks like `onclick="..."` can also be used in the normal way if you don't want any context for your execution: `<button onclick="alert('hello')">Button</button>`.

If you want a callback to call a function in your element class, put an exclamation mark in front of the callback name: `<button !onclick="this.someCustomFunction()">Button</button>`.

You can give parameters to instantiated elements as if it was a normal element. For example, `<MyElem name="John" />` will set `this.name` on the `MyElem` instance. That is how to make plain text parameters, to make evaluated/integer/object parameters put an exclamation mark at the start of the parameter name: `<MyElem !number=5 />` . The context of parameter evaulation is within a closure defined in a method of the element class.

## Roadmap

#### Public changes

- Allow there to be no `common/` dir, etc
- Give better error messages if required files (meta, elements) don't exist
- Maybe a namespace/build/import system for `common/` files?
    - Perhaps this should be left until we get a proper bundler
- Some sort of system for passing what Blazor calls render fragments - allows templating of tables and stuff
- Consistent "special chars" - don't use a tilde over here and a exclamation mark over there, and ${} string interpolation here. Make the markup consistent (like how Razor uses the @ sign for everything).
- Make route parameters for pages like `product/{id}`. 
- Ability to keep references to html elements
- Move to typescript, attach some sort of package manager
    - Build a standard library of components
- Allow subdirectories of elements + pages, something namespace-like
- Data binding/two-way parameters
- Scoped CSS
- Make requests to non-index directories still lead to the SPA (is this possible without writing a custom server?)
- Make project-template-creater (similar to `dotnet new`)
- Make custom dev server with file watching (similar to `dotnet watch run`)

#### Internal changes

- Skip empty content tags.
- Rewrite tokeniser to make tokens smaller. For example one token would be a single `<` instead of a whole tag. This makes it way easier to add consistent special chars.
    - Add an intermediate step to form individual tokens into stuff like tags.
- Restructure runtime stuff so that router is not a member of renderer, they are both members of an App
- Restructure runtime stuff so that multiple Spall apps can live on one page (currently uses statics)
- Restructure runtime stuff to auto-start? (or maybe it is preferable to manually create an app and attach it to the DOM)