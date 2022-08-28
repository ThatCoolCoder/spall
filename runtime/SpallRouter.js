class SpallRouter {
    // Handles switching between "pages" I guess.
    // Most of the work is done in the element, this just links everything together and adds a nice interface
    // Currently routes are just strings with no slashes in them. Proper urls will come when I add namespaces

    constructor(renderer) {
        this.renderer = renderer;

        this.routeToPageClass = {}; // define routes through here, by creating a page class
        this.crntRoute = ""; // empty route == homepage
    }

    navigateTo(route) {
        if (Object.keys(this.routeToPageClass).includes(route)) {
            this.crntRoute = route;
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