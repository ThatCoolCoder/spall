requires(SpallUtils.js);

class SpallRenderer {
    constructor(spallApp=null, logger=new SpallMockRenderLogger()) {
        this.spallApp = spallApp;
        this._logger = logger;

        this._lastUsedId = 0;
        this._rendering = false;
        
        this._idToHtml = {};
        this._idToPath = {}; // these two are relative to document.body
        this._pathToId = {};
        this._idToElement = {};
    }

    attachSpallApp(spallApp) {
        this.spallApp = spallApp;
    }

    renderPage() {
        this._throwIfRendering();
        if (this.spallApp == null) SpallUtils.fatalError("SpallRenderer.spallApp was not provided");
        var root = new __SpallCompiledRoot(this._lastUsedId, -1, this);
        this._idToHtml[root.id] = this.spallApp.appContainer;

        this._registerElement(root, '');

        try {
            this.renderElement(root, this.spallApp.appContainer);
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
        // list of [{htmlId: "", child: someSpallElem, parameters: {}}]
        var createdElements = [];

        for (var renderable of renderables) {
            if (renderable instanceof SpallMarkupRenderable) {
                finalHtml += renderable.markup;
                this._logger.logAddMarkup(renderable.markup);
            }
            else {
                var path = this._idToPath[element.id] + '/' + renderable.relativePath;
                var child = new renderable.elementClass(this._newElementId(), element.id, this.spallApp, path);

                var id = "__sp" + child.id;
                var className = `_sp${child.elementName}`;
                finalHtml += `<span style="display: contents" class="${className}" id="${id}"></span>`;

                createdElements.push({htmlId: id, child: child, parameters: renderable.parameters});
                this._registerElement(child, child.path);

                this._logger.logCreatedElement(child);

            }
        }
        container.innerHTML = finalHtml;

        for (var toRender of createdElements) {
            var childContainer = document.getElementById(toRender.htmlId);

            this._idToHtml[toRender.child.id] = childContainer;
            toRender.child.onInitialized();

            for (var parameterName in toRender.parameters) {
                toRender.child[parameterName] = toRender.parameters[parameterName]();
            }

            this.renderElement(toRender.child, childContainer);
        }
        
        this._logger.logFinishRender(element);

        element.onRender();
    }

    getElementContainer(elementId) {
        return this._idToHtml[elementId];
    }

    getElementById(elementId) {
        return this._idToElement[elementId];
    }

    getElementByPath(elementPath) {
        return this._idToElement[this._pathToId[elementPath]];
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
        if (this._rendering) throw new Exception("Already rendering");
    }

    _numericIdToHtmlId(id) {
        return `__sp${id}`;
    }

    _getHtmlElementByPath(path, baseElement=document.body) {
        // Path is relative to base element, which defaults to root of app
        // (this corresponds to the commented out lines) Paths are in the format of /div.2/p.1/b.1 
        // (this is correct) Paths are in the format of /0/4/2 - just child index
        // note that this refers to HTML elements

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