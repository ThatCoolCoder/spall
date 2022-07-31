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