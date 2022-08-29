
        class __SpallCompiledButton extends SpallElement {
            constructor(id, parentId, rendererInstance, path) {
                super('Button', id, parentId, rendererInstance, path);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button onclick="alert('You have clicked me!')"(callback) ><span >Im a button</span></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor(id, parentId, rendererInstance, path) {
                super('Root', id, parentId, rendererInstance, path);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<div style="width: 100%; background-color: blue"><h1 ><span >Demo App</span></h1><button onclick="SpallRenderer.instance.router.navigateTo('')"><span >Home</span></button><button onclick="SpallRenderer.instance.router.navigateTo('weather')"><span >Weather</span></button></div>`), new SpallElementRenderable("RoutedApp", __SpallCompiledRoutedApp, "1", {  })]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledStyledButton extends SpallElement {
            constructor(id, parentId, rendererInstance, path) {
                super('StyledButton', id, parentId, rendererInstance, path);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button style="background-color: black; color: white"><span >I'm a styled button</span></button>`)]);
return __spallRenderables;
            }

            
        }
    

        class __SpallCompiledCounterButton extends SpallElement {
            constructor(id, parentId, rendererInstance, path) {
                super('CounterButton', id, parentId, rendererInstance, path);
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<button onclick="SpallRenderer.instance.getElementById(${this.id}).count()"><span >Clicked ${this.formatCount()}</span></button>`)]);
return __spallRenderables;
            }

            
    onInitialized() {
        this.counter = 0;
    }

    count() {
        this.counter ++;
        this.needsRender();
    }

    formatCount() {
        return `${this.counter} ${this.counter == 1 ? 'time' : 'times'}`;
    }

        }
    

        class __SpallCompiledRenderCounter extends SpallElement {
            constructor(id, parentId, rendererInstance, path) {
                super('RenderCounter', id, parentId, rendererInstance, path);
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
    

        class __SpallCompiledIndex extends SpallPage {
            constructor(id, parentId, rendererInstance, path) {
                super('Hello', 'Index', id, parentId, rendererInstance, path)
            }

            generateRenderables() {
                var __spallRenderables = [];
// currently just a mockup of how this would look
__spallRenderables.push(...[new SpallMarkupRenderable(`<pageroute ></pageroute><h1 ><span >Basic Spall Demo</span></h1><div ><p ><span >So here we have some text</span></p><p ><span >And here is an instantiated element: </span>`), new SpallElementRenderable("Button", __SpallCompiledButton, "4/1/1", {  }), new SpallMarkupRenderable(`</p><p ><span >This instantiated button has some styling: </span>`), new SpallElementRenderable("StyledButton", __SpallCompiledStyledButton, "4/2/1", {  }), new SpallMarkupRenderable(`</p><p ><span >This instantiated button has a callback to itself, which changes the state: </span>`), new SpallElementRenderable("CounterButton", __SpallCompiledCounterButton, "4/3/1", { counter:() => 5 }), new SpallMarkupRenderable(`</p>`), new SpallElementRenderable("RenderCounter", __SpallCompiledRenderCounter, "4/4", {  }), new SpallMarkupRenderable(`<p ><span >The next sentence is generated on the fly with an if-statement</span></p><span >`)]);
if (Math.random() > 0.5) {
__spallRenderables.push(...[new SpallMarkupRenderable(`<p ><span >Math.random() was lower than 0.5</span></p>`)]);
} else {
__spallRenderables.push(...[new SpallMarkupRenderable(`<p ><span >Math.random() was greater than 0.5</span></p>`)]);
}
__spallRenderables.push(...[new SpallMarkupRenderable(`</span></div>`)]);
return __spallRenderables;
            }

            
        }
    SpallRouter.routeToPageClass[''] = __SpallCompiledIndex;


        class __SpallCompiledWeather extends SpallPage {
            constructor(id, parentId, rendererInstance, path) {
                super('Weather page', 'Weather', id, parentId, rendererInstance, path)
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<pageroute >weather</pageroute><p ><span >There is weather yes</span></p>`)]);
return __spallRenderables;
            }

            
        }
    SpallRouter.routeToPageClass['weather'] = __SpallCompiledWeather;
