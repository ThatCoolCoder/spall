class SpallUtils {
    static fatalRenderError(message) {
        console.error(`Fatal renderer error: ${message}`);
    }
}

class SpallElement {
    // Represents an element that's actually on the page and has a state and such. Is extended by compiled files.
    constructor(elementName, id, parentId) {
        this.elementName = elementName;
        this.id = id;
        this.parentId = parentId;
        this.children = [];
    }

    // Should return an array of SpallRenderables
    generateRenderables() {
        SpallUtils.fatalRenderError(`generateRenderables was not overridden in ${this.constructor.name}`);
    }
}

class SpallRootElement extends SpallElement {

}

// class __CompiledExampleElement extends SpallElement {
//     generateRenderables() {
//         return ['<h1>', 55, '</h1>'];
//     }
// }

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
    constructor(elementName, elementClass) {
        super();
        this.elementName = elementName;
        this.elementClass = elementClass;
    }
}

class SpallRenderLogger {
    constructor() {
        this.indent = 0;
        this.indentIncrement = 3;
    }

    logStartRender(element) {
        console.log(`${this._generateIndent()}-- Start render ${element.elementName}`);
        this.indent += this.indentIncrement;
    }

    logAddMarkup(markup) {
        console.log(`${this._generateIndent()}Rendering ${markup}`);
    }

    logFinishRender(element) {
        this.indent -= this.indentIncrement;
        console.log(`${this._generateIndent()}-- Finish render ${element.elementName}`);
    }

    logCreatedElement(element) {
        console.log(`${this._generateIndent()}Creating element for ${element.elementName}. Id is ${element.id}`)
    }

    _generateIndent() {
        return ' '.repeat(this.indent);
    }
}

class SpallRenderer {
    constructor() {
        this._idToHtml = {};
        this._lastUsedId = 0;
        this.rendering = false;
        this.pageTree = {}; // 
        this._logger = new SpallRenderLogger();
    }
    
    renderPage() {
        this._throwIfRendering();
        this.renderElement(new __SpallCompiledRoot());
    }

    renderPath(element, path) {
        this._throwIfRendering();

        var fullPath = '';

    }

    renderPageSimple() {
        this._throwIfRendering();
        var root = new __SpallCompiledRoot(this._lastUsedId, -1);
        this._idToHtml[root.id] = document.body;
        this.renderSimple(root, document.body);
    }

    renderSimple(element, container) {
        this._logger.logStartRender(element);
        this._throwIfRendering();

        var renderables = element.generateRenderables();

        var finalHtml = '';

        for (var renderable of renderables) {
            if (renderable instanceof SpallMarkupRenderable) {
                finalHtml += renderable.markup;
                this._logger.logAddMarkup(renderable.markup);
            }
            else {
                var child = new renderable.elementClass(this._generateElemId(), element.id);

                var childContainer = document.createElement('div');
                childContainer.id = child.id;
                this._idToHtml[child.id] = childContainer;

                this.renderSimple(child, childContainer);

                document.body.appendChild(childContainer);

                this._logger.logCreatedElement(child);

                finalHtml += childContainer.innerHTML;
            }
        }

        container.innerHTML = finalHtml;
        this._logger.logFinishRender(element);
    }
    
    renderElement(element) {
        this._throwIfRendering();


        var htmlElement = this._idToHtml[element.id];
        // existingHtml.parentElement.removeChild(existingHtml);
        if (htmlElement == null) {
            htmlElement = document.createElement('div');
            if (element instanceof SpallRootElement) var parent = document.body;
            else var parent = this._idToHtml[element.parentId];
            parent.appendChild(htmlElement);
        }

        var renderables = element.generateRenderables();
        htmlElement.innerHTML = renderables.join('');
    }

    _generateElemId() {
        return ++this._lastUsedId;
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