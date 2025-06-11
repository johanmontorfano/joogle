export function NewDomain() {
    return <div class="w-full">
        <div>
            <p class="step">
                <span class="step-count">1</span>
                Add Domain
            </p>
            <p>Provide your domain to begin</p>
            <form onSubmit={ev => ev.preventDefault()}>
                <input type="text" />
                <input type="submit" value="send" />
            </form>
        </div>
        <div>
            <p class="step">
                <span class="step-count">2</span>
                Confirm Ownership
            </p>
        </div>
        <div>
            <p class="step">
                <span class="step-count">3</span>
                Start Indexing
            </p>
        </div>
    </div>
}
