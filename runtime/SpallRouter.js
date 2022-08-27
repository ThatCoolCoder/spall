class SpallRouter {
    // Handles switching between pages I guess
    constructor(renderer) {
        this.renderer = renderer;
        this.routeToPageClass = {};
    }

    navigateTo(url) {
        // todo: look up in registry of pages, somehow splat an element inside of Root.spall
    }
}