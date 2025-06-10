export function Jobs() {
    async function onSubmit(ev: SubmitEvent) {
        ev.preventDefault();

        const url = ev.target[0].value;
        const res = await (await fetch("/index/urls", {
            method: "POST",
            body: JSON.stringify([url])
        })).text();
    
        console.log(res);
    }

    return <div>
        <h1>Jobs</h1>
        <div>
            <div>
                <form onSubmit={onSubmit}>
                    <input type="url" name="url" />
                    <input type="submit" value="Index !" />
                </form>
            </div>
        </div>
    </div>
}
