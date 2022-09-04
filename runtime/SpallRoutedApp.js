requires(SpallElement.js);

class __SpallCompiledRoutedApp extends SpallElement {
    // Defines the section of the app that is rendered by routing
    // I'm too lazy to make a proper system for predefined elements or imports, so it's just a manually compiled element

    constructor(id, parentId, spallApp, path) {
        super("RoutedApp", id, parentId, spallApp, path);
    }

    generateRenderables() {
        var elementClass = this.spallApp.router.getElementForRoute();
        return [new SpallElementRenderable("", elementClass, "1", {})];
    }
}