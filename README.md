[![Production Build](https://github.com/brimworks/somaeastside.org/actions/workflows/prod-build.yml/badge.svg)](https://github.com/brimworks/somaeastside.org/actions/workflows/prod-build.yml)

# Website Content Volunteers Welcome

Interested in helping to maintain our website *content*? If so, follow this simple process:

1. Visit a page on the [Soma](http://soma.brimworks.com) website
2. Scroll to the bottom of the page and click on the pencil icon in the page footer.
3. If you are not logged into github, you will need to create an account (or login).
4. Click on the "Fork this repository" button.
5. If the file being edited ends with `.md`, then this is a file in Markdown sytnax.
   To learn more about Markdown, please visit the [Markdown Cheat Sheet](https://www.markdownguide.org/cheat-sheet/).
6. If the file being edited ends with `.tsx`, then please talk with a tech volunteer
   to help edit that file.
7. When you have edited the file how you would like, click on the "Preview" button to
   provide a rough idea of what the published document will look like.
8. Click on "Propose changes" (optionally providing a brief description of your change).
9. Click on "Create pull request".
10. Click on "Create pull request" a second time.


When you click "save", a pull request
Talk to Brian about getting your content change approved and "merged"

# Tech Volunteers Welcome

This website is built with [Lume](https://lume.land/docs/overview/about-lume/) and dynamic content served
from an AWS Lambda exposed to the `/api` endpoint. This Lambda is built using [Rust](https://rustup.rs/)
and [Poem](https://docs.rs/poem) and interacts with the
[Planning Center API](https://developer.planning.center/docs/#/overview/).

Contributions in the form of pull requests are welcome, although please create an issue in github
before attempting any substantial changes so we can provide guidance.

When developing locally you will need to install some local dependencies first:

* `git` [How to install git](https://github.com/git-guides/install-git)
* `cargo` [Rustup is the recommended method](https://rustup.rs/)
* `deno` [How to install deno](https://docs.deno.com/runtime/manual/getting_started/installation)

1. Visit the [github repository](https://github.com/brimworks/somaeastside.org), and click the "Fork" button near the upper right corner.
2. Clone your fork of the repository. If you are new to this, please visit [setting up git](https://docs.github.com/en/get-started/getting-started-with-git/set-up-git).
3. `cd` into the directory of your fork and run `deno task serve`... or if you have Make installed, simply run `make run`.

If you intend to make changes to the dynamic content, `cd` into `lambda/pco` and run `make run` which will run the "PCO" (planning center online)
proxy lambda in the context of a local server.

TODO: Describe how to change the URL of the API to point at the localhost version.

