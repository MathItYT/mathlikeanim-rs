const express = require('express');
const cors = require('cors');
const app = express();

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

app.get('/beach-1751455_1920.jpg', (req, res) => {
    res.sendFile(__dirname + '/beach-1751455_1920.jpg');
});

app.get('/index.js', (req, res) => {
    res.sendFile(__dirname + '/index.js');
});

app.get('/tex2svg.js', (req, res) => {
    res.sendFile(__dirname + '/tex2svg.js');
});

app.listen(8080, () => {
    console.log('Server running on port 8080');
});
