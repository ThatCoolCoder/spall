
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
__spallRenderables.push(...[new SpallMarkupRenderable(`<div class="top-row px-2"><h1 class="main-title"><span >Demo App</span></h1><div class="d-flex">`), new SpallElementRenderable("NavButton", __SpallCompiledNavButton, "0/1/0", { title:() => "Home",route:() => "" }), new SpallMarkupRenderable(``), new SpallElementRenderable("NavButton", __SpallCompiledNavButton, "0/1/1", { title:() => "Counter World",route:() => "counterworld" }), new SpallMarkupRenderable(``), new SpallElementRenderable("NavButton", __SpallCompiledNavButton, "0/1/2", { title:() => "Weather",route:() => "weather" }), new SpallMarkupRenderable(`</div></div><div class="px-2 py-1">`), new SpallElementRenderable("RoutedApp", __SpallCompiledRoutedApp, "1/0", {  }), new SpallMarkupRenderable(`</div>`)]);
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
    

        class __SpallCompiledNavButton extends SpallElement {
            constructor(id, parentId, rendererInstance, path) {
                super('NavButton', id, parentId, rendererInstance, path);
            }

            generateRenderables() {
                var __spallRenderables = [];
if (this.selected) {
__spallRenderables.push(...[new SpallMarkupRenderable(`<a class="nav-button nav-button-selected px-4 py-2"><span >${this.title}</span></a>`)]);
} else {
__spallRenderables.push(...[new SpallMarkupRenderable(`<a class="nav-button px-4 py-2" onclick="SpallRenderer.instance.getElementById(${this.id}).visitLink()"><span >${this.title}</span></a>`)]);
}
return __spallRenderables;
            }

            
    onInitialized() {
        this.route = 'unrouted';
        this.title = 'Untitled';
    }

    get selected() {
        return window.location.pathname.replaceAll('/', '') == this.route;
    }

    visitLink() {
        this.renderer.router.navigateTo(this.route);
    }

        }
    

        class __SpallCompiledCounter extends SpallElement {
            constructor(id, parentId, rendererInstance, path) {
                super('Counter', id, parentId, rendererInstance, path);
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
    

        class __SpallCompiledIndex extends SpallPage {
            constructor(id, parentId, rendererInstance, path) {
                super('Hello', 'Index', id, parentId, rendererInstance, path)
            }

            generateRenderables() {
                var __spallRenderables = [];
__spallRenderables.push(...[new SpallMarkupRenderable(`<h1 ><span >Welcome to Spall</span></h1><p ><span >Spall is a Javascript framework for building SPAs, with a compiler written in Rust.</span></p><p ><span >Here is an example of basic interactive stateful behavior: </span>`), new SpallElementRenderable("Counter", __SpallCompiledCounter, "4/1", {  }), new SpallMarkupRenderable(`</p><p ><span >Click the links above to learn more about Spall</span></p>`)]);
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
__spallRenderables.push(...[new SpallMarkupRenderable(`<p ><span >There is weather yes</span></p>`)]);
return __spallRenderables;
            }

            
        }
    SpallRouter.routeToPageClass['weather'] = __SpallCompiledWeather;


        class __SpallCompiledCounterWorld extends SpallPage {
            constructor(id, parentId, rendererInstance, path) {
                super('Counter World', 'CounterWorld', id, parentId, rendererInstance, path)
            }

            generateRenderables() {
                var __spallRenderables = [];
for (var i = 0; i < 100; i ++) {
__spallRenderables.push(...[new SpallMarkupRenderable(``), new SpallElementRenderable("Counter", __SpallCompiledCounter, "2/0", {  })]);
}
__spallRenderables.push(...[new SpallMarkupRenderable(`<p ></p>`)]);
return __spallRenderables;
            }

            
        }
    SpallRouter.routeToPageClass['counterworld'] = __SpallCompiledCounterWorld;
