
        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('Button', id, parentId, rendererInstance);
            }

            generateRenderables() {
                return [new SpallMarkupRenderable(`<button><span>I'm a button</span></button>`)];
            }
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId, rendererInstance) {
                super('Root', id, parentId, rendererInstance);
            }

            generateRenderables() {
                return [new SpallMarkupRenderable(`<h1><span>Hello world!</span></h1><span>
</span><div><span>
    Before the button
    </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/1"), new SpallMarkupRenderable(`<span>
    After the button
    </span><br /><span>
    Another button
    </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/5"), new SpallMarkupRenderable(`<span>
    </span><p><span>Look, I'm about to break the escaper: "\`haha\`\\ </span></p><span>

    And look, there's a line</span><br /><span> break right in the middle of this sentence.
</span></div>`)];
            }
        }
    