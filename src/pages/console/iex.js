function make_text(content) {
    const el = document.createElement("p");

    el.innerText = content;
    return el;
}

async function ie_hydrate() {
    const ql = document.getElementById("ie__queue_length");
    const ql_res = await (await fetch("/api/get_queue_length")).text();

    ql.innerText = ql_res;

    const iu = document.getElementById("ie__indexed_urls");
    const iu_res = await (await fetch("/api/get_indexed_urls")).text();

    iu.innerText = iu_res;
}

setInterval(ie_hydrate, 1000);
