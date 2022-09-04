class SpallElement {
    // Represents an element that's actually on the page and has a state and such. Is extended by compiled files.
    constructor(elementName, id, parentId, spallApp, path) {
        this.elementName = elementName;
        this.id = id;
        this.parentId = parentId;
        this.children = [];
        this.spallApp = spallApp;
        this.path = path;
    }

    generateRenderables() {
        // Override this method to add custom behavior to element-derived things like pages
        return this.compiledGenerateRenderables();
    }

    // Override to specify renderables
    // Should return an array of SpallRenderables
    compiledGenerateRenderables() {
        SpallUtils.fatalRenderError(`generateRenderables was not overridden in ${this.constructor.name}`);
    }

    needsRender() {
        this.spallApp.renderer.renderElement(this, this.spallApp.renderer.getElementContainer(this.id));
    }

    onInitialized() {

    }

    onRender() {
        
    }
}

// Compiled project creates a bunch of classes like this one:

// class __CompiledExampleElement extends SpallElement {
//     compiledGenerateRenderables() {
//         return [new SpallMarkupRenderable('<h1>'), new SpallElementRenderable(...), new SpallMarkupRenderable('</h1>')];
//     }
// }