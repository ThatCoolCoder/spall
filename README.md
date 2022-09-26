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

The `runtime/` dir of the repo contains the stuff that runs in the browser. It contains multiple files which are bundled into the Rust executable and built using a custom import system. See inside one of the files to see how to import other files. `build.rs` makes the project rebuild if these are changed.

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
- Data binding/two-way parameters
- Scoped CSS
    - Will require interfering with `class=""` attributes of elements to map the names of classes
    - Alternately, could transform css to use selectors such as `__sp123 userdefinedclass`. The outer selector would be applied to the `<span>` containing the element.
    - Improve tokeniser
    - Make it write to a `css/` subdirectory of the build dir.
    - Switch to using element id instead of element name
    - Make changes to runtime so that the css will be applied
- Make requests to non-index directories still lead to the SPA (is this possible without writing a custom server?)
- Make project-template-creater (similar to `dotnet new`)
- Make custom dev server with file watching (similar to `dotnet watch run`)
- Add resilience for when JS lines don't end in a semicolon (they are broken by minifier)
- Prioritise direct route matches compared to parameter matches. Eg we can have a page `/users/me/` and a page `/users/{userId}` and if both match the first one is picked.
    - Can create a system of specificity that also works for wildcards.
    - Perhaps should treat wildcards and parameters in the same way
- Add support for types in route parameters is - currently it's all strings and you'll have to convert them yourself
    - This would likely be easier in typescript with generics
- Support for comments in HTML parser
    - Should they be included in the final markup? Let's add a compilation option for that, by default it will be no.
- Give a warning when multiple pages have the same route.
    - Add this and some other summary information to the page-compilation result struct.

#### Internal changes

- Rewrite tokeniser to make tokens smaller. For example one token would be a single `<` instead of a whole tag. This makes it way easier to add consistent special chars.
    - Add an intermediate step to form individual tokens into stuff like tags.
- Restructure runtime stuff so that multiple Spall apps can live on one page (currently uses statics)
    - Would be very difficult due to the slightly hacky way we give context for callbacks.
- Potentially move to a more object-oriented approach where tokens decide to compile themselves
- Maybe don't even bother rendering markup if it matches what was written before (actually, sounds hard). Would be desirable if adding auto-render after callbacks 
- Increase robustness of route parsing in rust (see associated functions in `file_compiler.rs` for details)
- Did we make pages missing be resilient?
