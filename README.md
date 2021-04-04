# braun-odw.eu

## About

Infrastructure for some of my websites. Built with a homegrown static site generator (because it's fun and educational) and hosted on Netlify.

## Testing

A local development server can be started using the [Netlify CLI](https://cli.netlify.com/):

``` bash
netlify dev
```

This will automatically open `http://localhost:8888/` in your browser, which shows nothing and returns a 404 response code. Add the domain of the site you want to see to that address. For example, the site that ends up being deployed to <a href="https://hanno.braun-odw.eu/">https://hanno.braun-odw.eu/</a> will be available at `http://localhost:8888/hanno.braun-odw.eu/`.
