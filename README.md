# Spall - A chip off the old block

> Spall: (noun) A chip or splinter of stone

Unremarkable SPA JS framework for the benefit of my learning, is probably very similar to some other frameworks. I've built this before using a JS framework so that I'm not influenced by the design of others. I have used C# Blazor extensively in the past. Compiler is built in Rust. Very WIP. In this initial version everything is fully custom - custom markup language, custom tokeniser, custom build/bundle tool.

See misc notes to see what this is currently capable of.

## Misc notes:

See `demos/basic` for a basic look at how to make stuff work.

An app is made up of elements, which are stored in the `elements/` directory. They're basically components but with a better name. Element file names are in format `{element name}.spall`. There must be an element called `Root` which is the root of your app. Calling them elements is somewhat confusing because HTML elements also exist. Maybe I should change the name.

The `pages/` dir of a project holds pages. Pages are just elements that correspond to a "route". Bad things will happen if a page and an element have the same name See the Routing section for more information.

The `meta/` dir of a project contains stuff that is not the app itself. `index.html` is the entry point into the app and is plain html. You should put stuff like linking to the renderer in there.

The `static/` dir of a project holds static files that can be accessed in the built app from `static/`.

The `common/` dir of a project holds Javascript files that can be accessed from elements and pages. Use it for shared functions or business logic - anything not directly tied to the frontend.

The `styles/` dir of a projects holds scoped CSS for elements. Code in `styles/Button.css` will only apply to markup in `elements/Button.spall` or `pages/Button.Spall`.

When an app is built, files are created in the `build/` directory, which can then be used in a regular server like Apache.

The `spallcomp/runtime/` dir of the repo contains the stuff that runs in the browser. It contains multiple files which are bundled into the Rust executable and built using a custom import system. See inside one of the files to see how to import other files. `build.rs` makes the project rebuild if these are changed.

The project for the compiler is located in `spallcomp/` and the project for the server of build apps is `spallserve` Currently the projects are separate binaries, in future I am to convert them into libraries and create a single binary with several subcommands (although this would require an argument parser supporting subcommands).


#### .spall markup format

This will change a lot in the future but this should be correct for some time:

It's pretty similar to HTML and uses the same element names. It aims to be as similar to HTML/JS as possible, reusing familiar symbols/concepts.

To interpolate/template values just do it like Javascript template literals: `<p>The value is ${Math.random()}</p>`. Interopolated values are evaluated in the context of one of the element's functions.

You can do conditionals and loops like this:
```
~if (Math.random() > 0.5) {~
    <p>It's a big number</p>
~} else {~
    <p>It's a small number</p>
~}~
```
Note that a closing tilde is optional if the Javascript ends at the end of a line. You can also run arbitrary JS code mid-render by writing something other than a conditional inside the tildes. This is useful for calculating intermediate values in complex calculations. The context of this code is inside a method of the generated class.

To add functions and state, add a script tag. Treat the script tag like an ES6 class wrapper, contents should look like:
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

You can give parameters to instantiated elements as if it was a normal element. For example, `<MyElem name="John" />` will set `this.name` on the `MyElem` instance. That is how to make plain text parameters, to make evaluated/integer/object parameters put an exclamation mark at the start of the parameter name: `<MyElem !number="5" />`. The context of parameter evaulation is within a closure defined in a method of the element class.

#### Routing

To add routing ("pages") to your app, first add a `<RoutedApp></RoutedApp>` element to your `Root.spall`. The routed portion of your app will be inserted inside there. Then you can write pages as `.spall` files in the `pages/` directory. Specify what route a page corresponds to using `<pageroute>your/route/here</pageroute>` tag. To navigate to that page from within an element (for example as a button-press callback), call `this.spallApp.router.navigateTo("your/route/here")`.

Specify route parameters using regular JS templating syntax. For example if you specify the route `/cars/${id}/info` and navigate to `/cars/55/info`, `this.id` will be set to `55` on the page. For more information, look at the `RouteParameters` page in `demos/basic`.

Wildcards can be used in page routes. Eg `<pageroute>animals/*</pageroute>` will be found if you navigate to both `animals/dogs` and `animals/cats`. It has the same lack of matching rules as a route parameter except that the value is not stored.

You can create multiple routes leading to the same page by simply specifying multiple `<pageroute>` tags.

Bad things may happen if you have multiple pages with the same route, so don't do that.

You can specify the title of a page within a `<title>` tag. Regular `${}` templating can be used in the title tag but it cannot contain other elements or interpolated javascript. To specify a default title for pages where the title is not provided, set `this.spallApp.router.defaultTitle = "Default Title Here"`. Note that if you want this to work for first render, you'll need to disable autorun in the `SpallApp` constructor, set the field, and then manually call `app.run()`.

## Roadmap

#### Public changes

- Auto-render after callbacks
- Distinguish between `!` callbacks and `!` parameters.
    - Do it by determining if it is an event?
        - Would work but would also be annoying if you want to bind an event to a lambda variable? 
            - People can work around that by making a function to do that so I think it's not a problem.
- Maybe a namespace/build/import system for `common/` files?
    - Perhaps this should be left until we get a proper bundler
- Some sort of system for passing what Blazor calls render fragments - allows templating of tables and stuff
- Consistent "special chars" - don't use a tilde over here and a exclamation mark over there, and ${} string interpolation here. Make the markup consistent (like how Razor uses the @ sign for everything).
- Ability to keep references to html elements
- Don't rebuild children elements when the parent is rerendered, only if the structure changes
    - First, we need to define what structures are and how to tell if two are equivalent.
- Move to typescript, attach some sort of package manager
    - Build a standard library of components
- Allow subdirectories of elements + pages, something namespace-like
    - Expand this to allow auto-deriving page name from folder and file structure
        - Still provide ways of adding a) custom last part of url; and b) custom entire url.
- Data binding/two-way parameters
- Scoped CSS
    - Put scoped css files in the same directories as markup files
        - Would be a great excuse to get a better method of finding the files to compile, and decentralizing from .spall files.
    - Improve tokeniser
        - Make it not remove comments
        - Make it more resilient to odd styling and also have more descriptive errors
    - Minify the scoped css and add command line flag to not do that
        - Should the flag be the same as the Javascript minify flag or different?
    - (Future) potentially add an option/insistence to use a CSS preprocessor
        - Depending on the preprocessor this would actually make it a whole lot easier to do it as they support nested rules so all we'd need to do is put a big rule over the top instead of tokenising.
        - But this would also require people to install more dependencies, which is annoying for them.
    - Make it write to a `css/` subdirectory of the build dir.
    - When we add subdirectories, make sure to make scope names include subdirectories
    - Alternately switch to using element id instead of element name
        - Requires compiling the CSS after so that element id is known
            - Except actually not 
        - Will allow us to give errors like complaining that a scoped css file doesn't have a matching element
            - This would require making `FileCompilationError` work for multiple types of files more easily.
    - Make changes to runtime so that the css will be applied
- Make project-template-creater (similar to `dotnet new`)
- Make a file watcher that runs spallcomp and spallserve 
- Add resilience for when JS lines don't end in a semicolon (they are broken by minifier)
- Prioritise direct route matches compared to parameter matches. Eg we can have a page `/users/me/` and a page `/users/{userId}/` and if both match the first one is picked.
    - Can create a system of specificity that also works for wildcards.
    - Perhaps should treat wildcards and parameters in the same way
- Add support for types in route parameters - currently it's all strings and you'll have to convert them yourself
    - This would likely be easier in typescript with generics
- Support for comments in HTML parser
    - Should they be included in the final markup? Let's add a compilation option for that, by default it will be no.
- Give a warning when multiple pages have the same route.
    - Add this and some other summary information to the page-compilation result struct.
        - Would possibly require improvement of route parsing and comparison in compiler, as currently all it does is convert straight to JS.

#### Internal changes

- Update hyper to only use required features.
- spallserve: use cache
- spallserve: favicon causes internal server error
- Rewrite tokeniser to make tokens smaller. For example one token would be a single `<` instead of a whole tag. This makes it way easier to add consistent special chars.
    - Add an intermediate step to form individual tokens into stuff like tags.
        - I think that's called lexing
- Restructure runtime stuff so that multiple Spall apps can live on one page (currently uses statics)
    - Would be very difficult due to the slightly hacky way we give context for callbacks.
- Potentially move to a more object-oriented approach where tokens decide to compile themselves
- Maybe don't even bother rendering markup if it matches what was written before (actually, sounds hard). Would be desirable if adding auto-render after callbacks 
- Increase robustness of route parsing in rust (see associated functions in `element_compiler.rs` for details)
- Did we make pages missing be resilient?
- Get a proper system of ids for precompiled/special elements
    - Perhaps switch element ids to be based on a hash of the element name and subdirectory
    - or GUID
- Make the `.spall` format not be the focus of everything - have a project compiler which does a number of tasks, only one of which is compiling the `.spall` files. Makes it easier to add more processing
    - Generally improve the error hierarchy to suppport this
- Comment code
- Proper documentation of the different data structures, terms, concepts and processes used.
- Generally split stuff up into more files which are each more focused.
    - Would make it easier to have sub-types of errors displayed with a good visual hierarchy
- Annoyingly there appear to be 2 other programs with the name Spall (even though I searched on Github before choosing the name!) so this name may just have to be a working one.
    - One of them is a MIDI player written in Ruby, and is of no concern
    - The other one is a `WASM flamegraph tracing renderer` written in Odin, which is of more concern because it's related to the web.
    - Potentially the name Spalljs/Spall.js is sufficiently unique.
    - At the end of the day I'm not building this for the purpose of being used so it doesn't matter than much
- Probably need to clean build dir before compilation
    - Static dir already is cleaned
- Test what happens if there is a circular dependency between runtime files.
    - See if we need to do something to warn about that
    - Not huge priority because it will always happen on the dev's machine, it's not dependent on client project (therefore should be easily spotted in testing)
- Refactor `.spall` tokeniser to use some of the tokenisation util functions that I added for the CSS tokeniser
- Add more tests, especially to tokenisers
