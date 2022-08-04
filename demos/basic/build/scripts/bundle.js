
        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('Button', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button ><span >I'm a button</span></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId, rendererInstance) {
                super('Root', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<h1 ><span >Basic Spall Demo</span></h1><span >
</span><div ><span >
    </span><p ><span >So here we have some text</span></p><span >
    </span><p ><span >And here is an instantiated element: </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "2/3/1"), new SpallMarkupRenderable(`</p><span >
    </span><p ><span >This instantiated button has some styling: </span>`), new SpallElementRenderable("StyledButton", __SpallCompiledStyledButton, "2/5/1"), new SpallMarkupRenderable(`</p><span >

    </span><p ><span >The next sentence is generated on the fly with an if-statement</span></p><span >
    </span>`)]);
if (Math.random() > 0.5) {
__spallRenderables.push(...[new SpallMarkupRenderable(`<span >        </span><p ><span >Math.random() was lower than 0.5</span></p><span >
    </span>`)]);
} else {
__spallRenderables.push(...[new SpallMarkupRenderable(`<span >        </span><p ><span >Math.random() was greater than 0.5</span></p><span >
    </span>`)]);
}
__spallRenderables.push(...[new SpallMarkupRenderable(`<span >    </span>`), new SpallElementRenderable("RenderCounter", __SpallCompiledRenderCounter, "2/9/5"), new SpallMarkupRenderable(`<span >
</span>`)]);
__spallRenderables.push(...[new SpallMarkupRenderable(`</div>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledStyledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('StyledButton', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button style="background-color:black;color:white"><span >I'm a styled button</span></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledRenderCounter extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('RenderCounter', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<div style="width:50px;height:50px;background-color:red"><span >Render counter</span></div><span >

</span>`)]);
return __spallRenderables;
            }

            
    onInitialized() {
        this.renderCounter = 0;
        setInterval(() => this.needsRender(), 5000);
    }

    onRender() {
        this.renderCounter ++;
        console.log(`Rendered ${this.renderCounter} times`);
    }

        }
    