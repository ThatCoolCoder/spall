
        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('Button', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button><span>I'm a button</span></button>`)]);
return __spallRenderables;
            }
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId, rendererInstance) {
                super('Root', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<h1><span>Hello world!</span></h1><span>
</span><div><span>
    Before the button
    </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/1"), new SpallMarkupRenderable(`<span>
    After the button
    </span><br /><span>
    Another button
    </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/5"), new SpallMarkupRenderable(`<span>
    </span><p><span>Look, I'm about to break the escaper: "\`haha\`\\ </span></p><span>

    And look, there's a line</span><br /><span> break right in the middle of this sentence.

    </span><p><span>Maybe this next para will not be there:</span></p><span>
    </span>`)]);
if (Math.random() > 0.5) {
__spallRenderables.push(...[new SpallMarkupRenderable(`<span>        </span><p><span>I'm here!</span></p><span>
    </span>`)]);
} else {
__spallRenderables.push(...[new SpallMarkupRenderable(`<span>        </span><p><span>I'm not here!</span></p><span>
    </span>`)]);
}
__spallRenderables.push(...[new SpallMarkupRenderable(`</div>`)]);
return __spallRenderables;
            }
        }
    