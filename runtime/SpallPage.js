requires(SpallElement.js);

class SpallPage extends SpallElement {
    constructor(title, elementName, id, parentId, renderer, path) {
        super(elementName, id, parentId, renderer, path);
        this.title = title;
    }

    generateRenderables() {
        document.title = this.title == "" ? this.renderer.router.defaultTitle : this.title;
        return this.compiledGenerateRenderables();
    }
}