
        class __SpallCompiledRenderCounter extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('RenderCounter', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<div style="width:200px;height:100px;background-color:red"><span >
Render counter
</span><br /><span >
I've rendered ${this.renderCounter + 1} times
</span></div>`)]);
return __spallRenderables;
            }

            
    onInitialized() {
        this.renderCounter = 0;
        setInterval(() => this.needsRender(), 5000);
    }

    onRender() {
        this.renderCounter ++;
    }

        }
    

        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('Button', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button onclick="alert('You have clicked me!')"><span >Im a button</span></button>`)]);
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
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId, rendererInstance) {
                super('Root', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<h1 ><span >Basic Spall Demo</span></h1><div ><p ><span >So here we have some text</span></p><p ><span >And here is an instantiated element: </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "1/1/1"), new SpallMarkupRenderable(`</p><p ><span >This instantiated button has some styling: </span>`), new SpallElementRenderable("StyledButton", __SpallCompiledStyledButton, "1/2/1"), new SpallMarkupRenderable(`</p><p ><span >The next sentence is generated on the fly with an if-statement</span></p>`), new SpallElementRenderable("RenderCounter", __SpallCompiledRenderCounter, "1/4"), new SpallMarkupRenderable(`<span >`)]);
if (Math.random() > 0.5) {
__spallRenderables.push(...[new SpallMarkupRenderable(`<p ><span >Math.random() was lower than 0.5</span></p>`)]);
} else {
__spallRenderables.push(...[new SpallMarkupRenderable(`<p ><span >Math.random() was greater than 0.5</span></p>`)]);
}
__spallRenderables.push(...[new SpallMarkupRenderable(`</span></div>`)]);
return __spallRenderables;
            }

            
        }
    