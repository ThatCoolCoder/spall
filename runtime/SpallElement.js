class SpallElement {
    // Represents an element that's actually on the page and has a state and such. Is extended by compiled files.
    constructor(elementName, id, parentId, rendererInstance) {
        this.elementName = elementName;
        this.id = id;
        this.parentId = parentId;
        this.children = [];
        this.rendererInstance = rendererInstance;
    }

    // Should return an array of SpallRenderables
    generateRenderables() {
        SpallUtils.fatalRenderError(`generateRenderables was not overridden in ${this.constructor.name}`);
    }

    needsRender() {
        this.rendererInstance.renderElement(this, this.rendererInstance.getElementContainer(this.id));
    }

    onInitialized() {

    }

    onRender() {
        
    }
}

// Compiled project creates a bunch of classes like this one:

// class __CompiledExampleElement extends SpallElement {
//     generateRenderables() {
//         return ['<h1>', new SpallElementRenderable(...), '</h1>'];
//     }
// }

class SpallRootElement extends SpallElement {

}