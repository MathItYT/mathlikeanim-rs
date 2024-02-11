export async function tex2svg(tex) {
    const url = `http://localhost:8080/tex2svg?from=${encodeURIComponent(tex)}`
    const svg = await fetch(url).then(res => res.text());
    return svg;
}
