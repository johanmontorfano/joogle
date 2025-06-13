/* @refresh reload */
import "./index.css";
import "../public/root.css";

import { Route, Router } from "@solidjs/router";
import { render } from "solid-js/web";
import { RootLayout } from "./pages/layout";
import { RootIndex } from "./pages";
import { Auth } from "./pages/auth";
import { NewDomain } from "./pages/new";
import { DomainView } from "./pages/domain";

const root = document.getElementById("root");

export const API_ENDPOINT = import.meta.env.PROD ? 
    import.meta.env.VITE_JOOGLE_API_ENDPOINT :
    import.meta.env.VITE_JOOGLE_API_ENDPOINT_DEV;

render(
    () => <Router>
        <Route path="/search/console" component={RootLayout}>
            <Route path="/" component={RootIndex} />
            <Route path="/new" component={NewDomain} />
            <Route path="/:domain" component={DomainView} />
        </Route>
        <Route path="/search/console/auth" component={Auth} />
    </Router>,
    root,
);
