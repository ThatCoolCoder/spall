requires(SpallElement.js);

class SpallPage extends SpallElement {
    constructor(elementName, id, parentId, spallApp, path) {
        super(elementName, id, parentId, spallApp, path);
    }

    generateRenderables() {
        document.title = this.generateTitle() || this.spallApp.router.defaultTitle;
        return this.compiledGenerateRenderables();
    }
}