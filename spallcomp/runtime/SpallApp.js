requires(SpallRenderer.js, SpallRouter.js);

// Root-most class of the Spall runtime

class SpallApp {
    constructor(options={}) {
        /*
            Example of options (all are optional):
            {
                appContainer: <html element>, (ignored if renderer is provided)
                renderer: <SpallRenderer>,
                router: <SpallRouter>,
                disableAutoRun: bool, (if is true, will not render immediately after construction)
            } 
        */
            
        if (Object.hasOwn(options, "appContainer")) {
            this.appContainer = options.appContainer;
        }
        else {
            this.appContainer = document.body;
        }

        if (Object.hasOwn(options, "renderer")) {
            this.renderer = renderer;
            this.renderer.attachSpallApp(this);
        }
        else {
            this.renderer = new SpallRenderer(this);
        }
        
        if (Object.hasOwn(options, "router")) {
            this.router = router;
            this.router.attachSpallApp(this);
        }
        else {
            this.router = new SpallRouter(this);
        }

        this.running = false;
        if (! options.disableAutoRun) {
            this.run();
        }

    }

    run() {
        this.running = true;
        this.renderer.renderPage();
    }
}