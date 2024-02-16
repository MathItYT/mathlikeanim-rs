export async function mathlike2svg() {
    const url = `http://localhost:8080/mathlike`
    const svg = await fetch(url).then(res => res.text());
    return svg;
}
