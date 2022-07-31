class SpallRenderable {
    // (abstract class thingy)
}

class SpallMarkupRenderable extends SpallRenderable {
    constructor(markup) {
        super();
        this.markup = markup;
    }
}

class SpallElementRenderable extends SpallRenderable {
    constructor(elementName, elementClass, relativePath) {
        super();
        this.elementName = elementName;
        this.elementClass = elementClass;
        this.relativePath = relativePath;
    }
}