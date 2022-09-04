requires(SpallElement.js);

class __SpallCompiledRoutedApp extends SpallElement {
    // Defines the section of the app that is rendered by routing
    // I'm too lazy to make a proper system for predefined elements or imports, so it's just a manually compiled element

    constructor(id, parentId, spallApp, path) {
        super("RoutedApp", id, parentId, spallApp, path);
    }

    compiledGenerateRenderables() {
        var stringRouteSections = this._parseStringRoute(this.spallApp.router.crntRoute);
        var matchingRouteData = null;
        for (var routeData of this.spallApp.router.routeList) {
            var [routeSections, elementClass] = routeData;
            if (this._stringMatchesRoute(stringRouteSections, routeSections)) {
                matchingRouteData = routeData;
                break;
            }
        }

        if (matchingRouteData == null) {
            return [new SpallMarkupRenderable("<p>The page you are looking for does not exist</p>")]
        }
        else {
            var [routeSections, elementClass] = matchingRouteData;
            var properties = {};
            stringRouteSections.forEach((crntStringSection, i) => {

                var crntRouteSection = routeSections[i];
                if (crntRouteSection instanceof SpallPropertyRouteSection) {
                    properties[crntRouteSection.propertyName] = () => crntStringSection;
                }
            });
            return [new SpallElementRenderable("", elementClass, "1", properties)];
        }

    }

    _parseStringRoute(stringRoute) {
        // Parse a string route into a list of sections
        return stringRoute.split('/').filter(x => x.length > 0);
    }

    _stringMatchesRoute(stringRouteSections, routeSections) {
        if (stringRouteSections.length != routeSections.length) return false;

        for (var i = 0; i < stringRouteSections.length; i ++) {
            var crntStringSection = stringRouteSections[i];
            var crntRouteSection = routeSections[i];
            if (crntRouteSection instanceof SpallPropertyRouteSection) {} // property route sections always match - that's how properties work!
            else if (crntRouteSection.value != crntStringSection) return false; // immediately we can tell it's not a match
        }
        return true;
    }
}