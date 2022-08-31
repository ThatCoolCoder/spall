class SpallRouter {
    // Handles switching between "pages" I guess.
    // Most of the work is done in the element, this just links everything together and adds a nice interface
    // Currently routes are just strings with no slashes in them. Proper urls will come when I add namespaces

    constructor(renderer) {
        this.renderer = renderer;

        this.routeToPageClass = SpallRouter.routeToPageClass;
        this.crntRoute = ""; // empty route == homepage
        this.defaultTitle = ""; // title shown if page doesn't define a title
    }

    setDefaultTitle(title) {
        this.defaultTitle = title;
    }

    navigateTo(route) {
        if (Object.keys(this.routeToPageClass).includes(route)) {
            this.crntRoute = route;
            history.pushState("", "", `/${this.crntRoute}`);
            this.renderer.renderPage();
        }
        else {
            throw new Error(`Cannot navigate to "${route}": route does not exist`);
        }
    }
    
    getElementForRoute() {
        return this.routeToPageClass[this.crntRoute];
    }
}

SpallRouter.routeToPageClass = {};