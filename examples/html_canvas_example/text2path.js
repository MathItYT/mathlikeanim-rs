export async function text2path(text) {
    const response = await fetch(`/text2path?text=${text}`);
    const svg = await response.text();
    return svg;
}
