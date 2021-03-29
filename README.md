# braun-odw.eu

## About

My personal IT infrastructure. Currently a work in progress.

## Testing

A local development server can be started using the [Netlify CLI](https://cli.netlify.com/):

``` bash
netlify dev
```

However, since there is some particular configuration around domains, additional steps need to be taken to make it work.

1. Edit `/etc/hosts` to point the domain you're testing to `127.0.0.1`.
2. In `netlify.toml`, change the redirect of the domain you're testing to redirect from `http://...` instead of `https://...`.
3. Make sure to add the port used by the Netlify Dev server to the URL in the browser.

Steps 1. and 2. will become redundant once we have a build process that can update the `<base>` element in the HTML document.

**TASK:** Is it possible to use a local development-only certificate to make step 2 redundant?

**TASK:** Can the Netlify Dev server be configured to use port 80, to make step 3 redundant? Can this be made to work from a permissions standpoint, without having to run everything as root?
