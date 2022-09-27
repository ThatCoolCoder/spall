class SpallUtils {
    static fatalError(message) {
        console.error(`Fatal Spall error:\n${this.indentText(message, this.errorIndent)}`);
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

    static indentText(text, numSpaces) {
        var indent = ' '.repeat(numSpaces);
        return text.split('\n').map(x => indent + x).join('\n');
    }

    static abstractNotOverridden() {
        // Put this in your abstract methods so it throws if it's not overridden.
        // Automatically figures out what the name of the function that called this is, for easy debugging.
        var functionName = new Error().stack.split('\n')[1].split('@')[0];
        throw new Error(`Abstract function "${functionName}" not overridden`);
    }
}

// I don't think safari likes static variables so just do it the dumb way
SpallUtils.errorIndent = 4;