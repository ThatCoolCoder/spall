
        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId) {
                super('Button', id, parentId);
            }

            generateRenderables() {
                return [new SpallMarkupRenderable(`<button>`), new SpallMarkupRenderable(`<span>I'm a button`), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`</button>`)];
            }
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId) {
                super('Root', id, parentId);
            }

            generateRenderables() {
                return [new SpallMarkupRenderable(`<h1>`), new SpallMarkupRenderable(`<span>Hello world!`), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`</h1>`), new SpallMarkupRenderable(`<span>
`), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`<div>`), new SpallMarkupRenderable(`<span>
    `), new SpallMarkupRenderable(`</span>`), new SpallElementRenderable("Button", __SpallCompiledButton), new SpallMarkupRenderable(`<span>
    Look, I'm about to break the escaper: "\`haha\`\\ 
`), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`</div>`)];
            }
        }
    