export async function mathlike2svg() {
    const url = `${window.location.href}mathlike`;
    const svg = await fetch(url).then(res => res.text());
    return svg;
}
