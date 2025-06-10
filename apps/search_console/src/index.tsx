/* @refresh reload */
import "./index.css";
import "../../../static/root.css";

import { Route, Router } from "@solidjs/router";
import { render } from "solid-js/web";
import { RootLayout } from "./pages/layout";
import "../public/root.css";
import "../public/search.css";

const root = document.getElementById("root");

render(
    () => <Router root={RootLayout}>
        <Route path="/" component={() => <p>Hello</p>} />
    </Router>,
    root,
);
