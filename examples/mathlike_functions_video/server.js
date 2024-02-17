const express = require('express');
const cors = require('cors');
const fs = require('fs');
const child_process = require('child_process');
const uuid = require('uuid');
const app = express();

if (!fs.existsSync('cache.json')) {
    fs.writeFileSync('cache.json', '{}');
}

if (!fs.existsSync('tex')) {
    fs.mkdirSync('tex');
}

app.use(cors(
    {
        origin: '*',
        optionsSuccessStatus: 200
    }
));

app.use('/pkg', express.static(__dirname + '/pkg'));

app.get('/', (req, res) => {
    res.sendFile(__dirname + '/index.html');
});

app.get('/matchSystemTheme.js', (req, res) => {
    res.sendFile(__dirname + '/matchSystemTheme.js');
});

app.get('/index.js', (req, res) => {
    res.sendFile(__dirname + '/index.js');
});

app.get('/tex2svg.js', (req, res) => {
    res.sendFile(__dirname + '/tex2svg.js');
});

app.get('/tex2svg', (req, res) => {
    let formula = decodeURIComponent(req.query.from);
    let cache = JSON.parse(fs.readFileSync('cache.json'));
    if (cache[formula]) {
        res.send(fs.readFileSync(cache[formula]));
        return;
    }
    let tex = String.raw`\documentclass[preview]{standalone}
\usepackage[spanish]{babel}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{mlmodern}
\usepackage{xcolor}

\begin{document}
\textcolor[HTML]{FFFFFF}{
    ${formula}
}
\end{document}`;
    let filename = uuid.v4();
    fs.writeFileSync('tex/' + filename + '.tex', tex);
    try {
        child_process.execSync(`latex -interaction=nonstopmode --shell-escape -halt-on-error --output-directory=tex tex/${filename}.tex`);
    } catch (e) {
        res.status(500).send('Error');
        return;
    }
    try {
        child_process.execSync(`dvisvgm tex/${filename}.dvi -n -o tex/${filename}.svg`);
    } catch (e) {
        res.status(500).send('Error');
        return;
    }
    let svg = fs.readFileSync('tex/' + filename + '.svg');
    cache[formula] = 'tex/' + filename + '.svg';
    fs.writeFileSync('cache.json', JSON.stringify(cache));
    res.send(svg);
});

app.get('/mathlike', (req, res) => {
    let svg = fs.readFileSync('mathlike.svg');
    res.send(svg);
});

app.get('/mathlike2svg.js', (req, res) => {
    res.sendFile(__dirname + '/mathlike2svg.js');
});

app.get('/style.css', (req, res) => {
    res.sendFile(__dirname + '/style.css');
});

app.get('/favicon.ico', (req, res) => res.sendFile(__dirname + '/favicon.ico'));

app.listen(8080, () => {
    console.log('Server running on port 8080');
});
