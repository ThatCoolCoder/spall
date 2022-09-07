class SpallRouter {
    // Handles switching between "pages" I guess.
    // Most of the work is done in the element, this just links everything together and adds a nice interface
    // Currently routes are just strings with no slashes in them. Proper urls will come when I add namespaces

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

    navigateTo(route) {
        this.crntRoute = route;
        history.pushState("", "", `/${this.crntRoute}`);
        if (this.spallApp == null) SpallUtils.fatalError("SpallRouter.attachApp() has not been called");
        this.spallApp.renderer.renderPage();
    }

    stringMatchesRoute(stringRouteSections, routeSections) {
        if (stringRouteSections.length != routeSections.length) return false;

        for (var i = 0; i < stringRouteSections.length; i ++) {
            var crntStringSection = stringRouteSections[i];
            var crntRouteSection = routeSections[i];
            if (crntRouteSection instanceof SpallPropertyRouteSection) {} // property route sections always match - that's how properties work!
            else if (crntRouteSection.value != crntStringSection) return false; // immediately we can tell it's not a match
        }
        return true;
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