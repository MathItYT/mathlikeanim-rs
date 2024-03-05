const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

app.get('/', (req, res) => {
    res.sendFile(__dirname + '/index.html');
});

app.get('/main.js', (req, res) => {
    res.sendFile(__dirname + '/main.js');
});

app.get('/pkg/*', (req, res) => {
    res.sendFile(__dirname + req.url);
});

app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});