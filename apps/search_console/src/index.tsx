/* @refresh reload */
import "./index.css";
import "../public/root.css";

import { Route, Router } from "@solidjs/router";
import { render } from "solid-js/web";
import { RootLayout } from "./pages/layout";
import { RootIndex } from "./pages";
import { Jobs } from "./pages/jobs";

const root = document.getElementById("root");

render(
    () => <Router root={RootLayout}>
        <Route path="/search/console" component={RootIndex} />
        <Route path="/search/console/jobs" component={Jobs} />
    </Router>,
    root,
);
