
        class __SpallCompiledButton extends SpallElement {
            constructor() {
                super('Button');
            }

            generateRenderables() {
                return [`<button>I'm a button</button>`];
            }
        }
    

        class __SpallCompiledRoot extends SpallRootElement {
            constructor() {
                super('Root');
            }

            generateRenderables() {
                return [`<h1>Hello world!</h1>
<Button></Button>
\`haha\`\\`];
            }
        }
    