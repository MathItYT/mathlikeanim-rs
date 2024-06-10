const express = require('express');
const app = express();

app.get('/*', (req, res) => {
    if (req.url === '/') {
        res.sendFile(__dirname + '/index.html');
    } else {
        res.sendFile(__dirname + req.url);
    }
});

app.listen(3000, () => {
    console.log('Server is running at http://localhost:3000');
});