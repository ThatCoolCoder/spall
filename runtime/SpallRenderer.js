class SpallRenderer {
    constructor(logger) {
        this._lastUsedId = 0;
        this.rendering = false;
        this._logger = logger;
        
        this._idToHtml = {};
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
                var path = this._idToPath[element.id] + '/' + renderable.relativePath;
                var child = new renderable.elementClass(this._newElementId(), element.id, this, path);

                var childContainer = document.createElement('span');
                childContainer.id = this._numericIdToHtmlId(child.id);
                this._idToHtml[child.id] = childContainer;

                this._registerElement(child, child.path);

                child.onInitialized();
                this.renderElement(child, childContainer);

                this._logger.logCreatedElement(child);

                needsAppending[renderable.relativePath] = childContainer;

            }
        }

        container.innerHTML = finalHtml;

        for (var renderablePath of Object.keys(needsAppending)) {
            var parentPath = renderablePath.split('/').slice(0, -1).join('/');
            var parent = this._getHtmlElementByPath(parentPath, container);
            SpallUtils.addChildAtIndex(parent, needsAppending[renderablePath], parseInt(renderablePath.split('/').slice(-1)));
        }
        
        this._logger.logFinishRender(element);

        element.onRender();
    }

    getElementContainer(elementId) {
        return this._idToHtml[elementId];
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
        if (this.rendering) throw new Exception("Already rendering");
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