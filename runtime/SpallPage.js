requires(SpallElement.js);

class SpallPage extends SpallElement {
    constructor(title, elementName, id, parentId, spallApp, path) {
        super(elementName, id, parentId, spallApp, path);
        this.title = title;
    }

    generateRenderables() {
        document.title = this.title == "" ? this.spallApp.router.defaultTitle : this.title;
        return this.compiledGenerateRenderables();
    }
}