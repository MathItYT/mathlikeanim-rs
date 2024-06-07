const express = require('express');
const app = express();
const { promisify } = require('util');
const fs = require('fs');
const { v4 } = require('uuid');
const { exec } = require('child_process');
const execAsync = promisify(exec);

if (!fs.existsSync(__dirname + '/temp')) {
    fs.mkdirSync(__dirname + '/temp');
}

if (!fs.existsSync(__dirname + '/cache.json')) {
    fs.writeFileSync(__dirname + '/cache.json', '{}');
}

if (!fs.existsSync(__dirname + '/frames')) {
    fs.mkdirSync(__dirname + '/frames');
}

app.get('/*', async (req, res) => {
    if (req.url === '/') {
        res.sendFile(__dirname + '/index.html');
    } else if (req.url.startsWith('/latex')) {
        let cache = JSON.parse(fs.readFileSync(__dirname + '/cache.json'));
        if (cache[req.query.input]) {
            return res.sendFile(__dirname + cache[req.query.input]);
        }
        const latex = decodeURIComponent(req.query.input);
        const filename = `/temp/${v4()}.tex`;
        const content = `
\\documentclass[preview]{standalone}
\\usepackage[spanish]{babel}
\\usepackage{amsmath}
\\usepackage{amssymb}

\\begin{document}
${latex}
\\end{document}
        `;
        fs.writeFileSync(__dirname + filename, content);
        await execAsync(`latex -interaction=nonstopmode ${__dirname + filename} --output-directory=${__dirname + '/temp'}`);
        await execAsync(`dvisvgm ${__dirname + filename.replace('.tex', '.dvi')} -n --output=${__dirname + filename.replace('.tex', '.svg')}`);
        if (!fs.existsSync(__dirname + filename.replace('.tex', '.svg'))) {
            return res.status(500).send('Error');
        }
        cache[req.query.input] = filename.replace('.tex', '.svg');
        fs.writeFileSync(__dirname + '/cache.json', JSON.stringify(cache));
        res.sendFile(__dirname + filename.replace('.tex', '.svg'));
    } else {
        res.sendFile(__dirname + req.url);
    }
});

app.listen(3000, () => {
    console.log('Server is running at http://localhost:3000');
});