class SpallElement {
    constructor(elementName, id, parentId) {
        this.elementName = elementName;
        // this.id = id;
        // this.parentId = parentId;
        // this.children = [];
    }

    generateRenderables() {

    }
}

class SpallRootElement extends SpallElement {

}

// class __CompiledExampleElement extends SpallElement {
//     generateRenderables() {
//         return ['<h1>', 55, '</h1>'];
//     }
// }

// class SpallRenderable {

// }

class SpallRenderer {
    constructor() {
        this.idToHtml = {};
        this.rendering = false;
        this.pageTree = {}; // 
    }
    
    renderPage() {
        this._throwIfRendering();
        this.renderElement(new __SpallCompiledRoot());
    }

    renderPath(element, path) {
        this._throwIfRendering();

        var fullPath = '';

    }

    renderSimple(element) {

    }
    
    renderElement(element) {
        this._throwIfRendering();


        var htmlElement = this.idToHtml[element.id];
        // existingHtml.parentElement.removeChild(existingHtml);
        if (htmlElement == null) {
            htmlElement = document.createElement('div');
            if (element instanceof SpallRootElement) var parent = document.body;
            else var parent = this.idToHtml[element.parentId];
            parent.appendChild(htmlElement);
        }

        var renderables = element.generateRenderables();
        htmlElement.innerHTML = renderables.join('');
    }


    _throwIfRendering() {
        if (this.rendering) throw new Exception("Already rendering");
    }

    _getElemFromPath(path) {

    }

    _numericIdToHtmlId(id) {
        return $`spallElement{id}`;
    }
}

SpallRenderer.instance = new SpallRenderer();