# braun-odw.eu

## About

Infrastructure for some of my websites. Built with a homegrown static site generator (because it's fun and educational) and hosted on Netlify.

## Testing

A local development server can be started using the [Netlify CLI](https://cli.netlify.com/):

``` bash
netlify dev
```

However, since there is some particular configuration around domains, additional steps need to be taken to make it work.

1. Edit `/etc/hosts` to point the domain you're testing to `127.0.0.1`.
2. In `netlify.toml`, change the redirect of the domain you're testing to redirect from `http://...` instead of `https://...`.
3. Make sure to add the port used by the Netlify Dev server to the URL in the browser.

All these steps will become redundant once we have a build process that can update the `<base>` element in the HTML document.
