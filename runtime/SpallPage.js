class SpallPage extends SpallElement {
    constructor(title, elementName, id, parentId, rendererInstance, path) {
        super(elementName, id, parentId, rendererInstance, path);
        this.title = title;
    }
}