const config = {
    method: 'GET',
    mode: 'cors',
    cache: 'no-cache',
    credentials: 'same-origin'
};


export async function tex2svg(tex) {
    const url = `http://localhost:8080/tex2svg?from=${encodeURIComponent(tex)}`
    const svg = await fetch(url, config).then(res => res.text());
    return svg;
}
