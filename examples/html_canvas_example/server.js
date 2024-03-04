const express = require('express');
const { load } = require('opentype.js');
const { readFileSync, writeFileSync, existsSync } = require('fs');
const { v4 } = require('uuid');
const app = express();
const PORT = process.env.PORT || 3000;


if (!existsSync('cache.json')) {
    writeFileSync('cache.json', '{}');
}

app.get('/', (req, res) => {
    res.sendFile(__dirname + '/index.html');
});

app.get('/main.js', (req, res) => {
    res.sendFile(__dirname + '/main.js');
});

app.get('/pkg/*', (req, res) => {
    res.sendFile(__dirname + req.url);
});

app.get('/text2path', async (req, res) => {
    const cache = JSON.parse(readFileSync('cache.json'));
    const text = req.query.text;
    if (cache[text]) {
        res.send(cache[text]);
        return;
    }
    const font = await load('fonts/Roboto-Bold.ttf');
    const path = font.getPath(text, 0, 150, 72);
    const svg = path.toSVG();
    cache[text] = svg;
    writeFileSync('cache.json', JSON.stringify(cache));
    res.send(svg);
});

app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});