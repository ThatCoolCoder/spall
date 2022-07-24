
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
    Before the button
    `), new SpallMarkupRenderable(`</span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/1"), new SpallMarkupRenderable(`<span>
    After the button
    `), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`<br />`), new SpallMarkupRenderable(`<span>
    Another button
    `), new SpallMarkupRenderable(`</span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/5"), new SpallMarkupRenderable(`<span>
    `), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`<p>`), new SpallMarkupRenderable(`<span>Look, I'm about to break the escaper: "\`haha\`\\ `), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`</p>`), new SpallMarkupRenderable(`<span>

    And look, there's a line`), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`<br />`), new SpallMarkupRenderable(`<span> break right in the middle of this sentence.
`), new SpallMarkupRenderable(`</span>`), new SpallMarkupRenderable(`</div>`)];
            }
        }
    