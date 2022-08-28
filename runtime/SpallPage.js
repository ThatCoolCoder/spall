requires(SpallElement.js);

class SpallPage extends SpallElement {
    constructor(title, elementName, id, parentId, renderer, path) {
        super(elementName, id, parentId, renderer, path);
        this.title = title;
    }
}