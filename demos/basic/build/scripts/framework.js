class SpallElement {
    constructor(id, parentId) {
        this.id = id;
        this.parentId = parentId;
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
    }
    
    renderPage() {
        this.renderElement(new __SpallCompiledRoot());
    }
    
    renderElement(element) {
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

    _numericIdToHtmlId(id) {
        return $`spallElement{id}`;
    }
}

SpallRenderer.instance = new SpallRenderer();