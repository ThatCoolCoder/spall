
        class __SpallCompiledRenderCounter extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('RenderCounter', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<div ttyle=idth:200px;height:100px;background-color:red">
Render counter
<br />
I've rendered ${this.renderCounter + 1} times
</div>

<script>
    onInitialized() {
        this.renderCounter = 0;
        setInterval(() => this.needsRender(), 5000);
    }

    onRender() {
        this.renderCounter ++;
    }
</script></div>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('Button', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button ></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledStyledButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('StyledButton', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button ttyle=ackground-color:black;color:white">I'm a styled button</button></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledScriptedButton extends SpallElement {
            constructor(id, parentId, rendererInstance) {
                super('ScriptedButton', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button nnclick=func>Scripted button</button>

<script>
    myfunc() {
        alert('wow it worked!');
    }
</script></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId, rendererInstance) {
                super('Root', id, parentId, rendererInstance);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<h1 ></h1>`)]);
return __spallRenderables;
            }

            
        }
    