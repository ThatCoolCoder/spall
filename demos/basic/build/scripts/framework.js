class SpallUtils {
    static fatalRenderError(message) {
        console.error(`Fatal renderer error: ${message}`);
    }

    static addChildAtIndex(element, child, index) {
        // Based on https://stackoverflow.com/a/39181175/12650706
        if (!index) index = 0
        if (index >= element.children.length) {
            element.appendChild(child)
        } else {
            element.insertBefore(child, element.children[index])
        }
    }

    static abstractNotOverridden() {
        // Put this in your abstract methods so it throws if it's not overridden.
        var functionName = new Error().stack.split('\n')[1].split('@')[0];
        throw new Error(`Abstract function "${functionName}" not overridden`);
    }
}

class SpallRenderer {
    constructor(logger) {
        this._idToHtml = {}; // d
        this._lastUsedId = 0;
        this.rendering = false;
        this._logger = logger;

        this._idToPath = {}; // these two are relative to document.body
        this._pathToId = {};
        this._idToElement = {};
    }
    
    renderPage() {
        this._throwIfRendering();
        var root = new __SpallCompiledRoot(this._lastUsedId, -1);
        this._idToHtml[root.id] = document.body;

        this._registerElement(root, '');

        try {
            this.renderElement(root, document.body);
        }
        catch (e) {
            SpallUtils.fatalRenderError(`General exception: ${e}\nStack trace: ${e.stack}`);
        }
    }

    renderElement(element, container) {
        this._logger.logStartRender(element);
        this._throwIfRendering();

        var renderables = element.generateRenderables();

        var finalHtml = '';
        // dictionary of relative path to html element
        var needsAppending = {};

        for (var renderable of renderables) {
            if (renderable instanceof SpallMarkupRenderable) {
                finalHtml += renderable.markup;
                this._logger.logAddMarkup(renderable.markup);
            }
            else {
                var child = new renderable.elementClass(this._newElementId(), element.id, this);

                var childContainer = document.createElement('div');
                childContainer.id = this._numericIdToHtmlId(child.id);
                this._idToHtml[child.id] = childContainer;

                this._registerElement(child, this._idToPath[element.id] + '/' + renderable.relativePath);

                this.renderElement(child, childContainer);


                this._logger.logCreatedElement(child);

                needsAppending[renderable.relativePath] = childContainer;

            }
        }

        container.innerHTML = finalHtml;

        for (var renderablePath of Object.keys(needsAppending)) {
            var parentPath = renderablePath.split('/').slice(0, -1).join('/');
            var parent = this._getElementByPath(parentPath, container);
            SpallUtils.addChildAtIndex(parent, needsAppending[renderablePath], parseInt(renderablePath.split('/').slice(-1)));
        }
        
        this._logger.logFinishRender(element);
    }

    _registerElement(element, path) {
        this._idToElement[element.id] = element;
        this._pathToId[path] = element.id;
        this._idToPath[element.id] = path;
    }

    _unregisterElement(element) {
        delete this._idToElement[element.id];
        var path = this._idToPath[element.id];
        delete this._pathToId[path];
        delete this._idToPath[element.id];
    }

    _newElementId() {
        return ++this._lastUsedId;
    }

    _throwIfRendering() {
        if (this.rendering) throw new Exception("Already rendering");
    }

    _numericIdToHtmlId(id) {
        return `__sp${id}`;
    }

    _getElementByPath(path, baseElement=document.body) {
        // Path is relative to base element, which defaults to root of app
        // (this corresponds to the commented out lines) Paths are in the format of /div.2/p.1/b.1 
        // (this is correct) Paths are in the format of /0/4/2 - just child index

        var sections = path.split('/');
        var crntElement = baseElement;
        for (var section of sections) {
            // if (section == '') continue;
            // var [tag, index] = section.split('.');
            // crntElement = [...crntElement.children].filter(c => c.tagName.toLowerCase() == tag)[index];
            crntElement = crntElement.children[parseInt(section)];
        }
        return crntElement;
    }
}
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
// Interface for render loggers
class ISpallRenderLogger {
    logStartRender(element) {
        SpallUtils.abstractNotOverridden();
    }

    logAddMarkup(markup) {
        SpallUtils.abstractNotOverridden();
    }

    logFinishRender(element) {
        SpallUtils.abstractNotOverridden();
    }

    logCreatedElement(element) {
        SpallUtils.abstractNotOverridden();
    }
}

// Render logger that does nothing, for production
class SpallMockRenderLogger {
    constructor() {
    }

    logStartRender(element) {

    }

    logAddMarkup(markup) {

    }

    logFinishRender(element) {

    }

    logCreatedElement(element) {

    }
}

// Render logger that logs everything, for development
class SpallDebugRenderLogger {
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
        this.rendererInstance.renderElement(this);
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