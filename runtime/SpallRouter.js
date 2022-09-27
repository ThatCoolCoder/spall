class SpallRouter {
    // Handles switching between "pages" I guess.
    // Most of the work is done in the element, this just links everything together and adds a nice interface
    // There are three-ish types of routes: string, parsed, and templates.
    // - String routes are just a string like "/myapp/hello/"
    // - Parsed routes are string routes that have been split at the slashes and cleaned to form something like ["myapp", "hello"]
    // - Template routes (also called routeSections, distinct from stringRouteSections) are a list of SpallStringRouteSections or SpallPropertyRouteSections.
    //      They are linked to a page and can be determined whether it matches the route that the user wants to navigate to.

    constructor(spallApp=null) {
        this.spallApp = spallApp;

        this.routeList = SpallRouter.routeList;
        this.crntRoute = ""; // empty route == homepage
        this.defaultTitle = ""; // title shown if page doesn't define a title
    }

    attachSpallApp(spallApp) {
        this.spallApp = spallApp;
    }

    setDefaultTitle(title) {
        this.defaultTitle = title;
    }

    // Navigate to a route 
    navigateTo(route) {
        this.crntRoute = route;
        history.pushState("", "", `/${this.crntRoute}`);
        if (this.spallApp == null) SpallUtils.fatalError("SpallRouter.attachApp() has not been called");
        this.spallApp.renderer.renderPage();
    }

    // Whether
    stringMatchesRoute(stringRouteSections, routeSections) {
        if (stringRouteSections.length != routeSections.length) return false;

        for (var i = 0; i < stringRouteSections.length; i ++) {
            var crntStringSection = stringRouteSections[i];
            var crntRouteSection = routeSections[i];
            if (crntRouteSection instanceof SpallPropertyRouteSection) {} // property route sections always match - that's how properties work!
            else if (crntStringSection == '*' || crntRouteSection.value == '*') {} // wildcard route matches everything
            else if (crntRouteSection.value != crntStringSection) return false; // immediately we can tell it's not a match
        }
        return true;
    }

    //
    stringRoutesMatch(route1, route2) {
        var route1Sections = this.parseStringRoute(route1);
        var route2Sections = this.parseStringRoute(route2);

        if (route1Sections.length != route2Sections.length) return false;

        for (var i = 0; i < route1Sections.length; i ++) {
            var crntRoute1Section = route1Sections[i];
            var crntRoute2Section = route2Sections[i];
            if (crntRoute1Section == '*' || crntRoute2Section == '*') {} // wildcard always matches
            else if (crntRoute1Section != crntRoute2Section) return false;
        }
        return true;
    }

    // 
    parseStringRoute(stringRoute) {
        // Parse a string route into a list of sections
        return stringRoute.split('/').filter(x => x.length > 0);
    }
}

// List of "tuples" of [<list of route sections>, page class]
SpallRouter.routeList = [];

class SpallStringRouteSection {
    constructor(value) {
        this.value = value;
    }
}

class SpallPropertyRouteSection {
    constructor(propertyName) {
        this.propertyName = propertyName;
    }
}