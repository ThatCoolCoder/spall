class SpallUtils {
    static fatalError(message) {
        console.error(`Fatal Spall error:\n    ${message}`);
    }

    static fatalRenderError(message) {
        this.fatalError(`Fatal renderer error:\n        ${message}`)
    }

    static addChildAtIndex(element, child, index) {
        // Based on https://stackoverflow.com/a/39181175/12650706
        if (!index) index = 0;
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