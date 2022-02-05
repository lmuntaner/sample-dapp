const compression = require('compression');
const polka = require('polka');
const { createProxyMiddleware } = require('http-proxy-middleware');
const sirv = require('sirv');

const { PORT, NODE_ENV } = process.env;
const port = PORT || 8080;
const dev = NODE_ENV === 'development';

polka()
  .use(
    compression({ threshold: 0 }),
    sirv('public', { dev }),
  )
  .use('/api', createProxyMiddleware({
    // localhost was not working
    // https://github.com/chimurai/http-proxy-middleware/issues/21
    target: 'http://127.0.0.1:8000/',
    changeOrigin: true,
  }))
  .listen(port || 8080, err => {
    if (err) console.log('error', err);
    else console.log(`Listening on localhost:${port}...`);
  });