import WebCarousel from '../templates/WebCarousel.tsx'
import Header from '../templates/Header.tsx'

//{% render 'templates/header.liquid' %}
export default ({ title, children }: Lume.Data, helpers: Lume.Helpers) => (
  <>
<html>
  <head>
    <title>{ title }</title>
    <meta charSet="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="stylesheet" href="/index.css" />
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto" />
  </head>
  <body className="dark:bg-black">
    <WebCarousel />
    <Header />
    <article className=" prose md:prose-lg dark:prose-invert max-w-none">
      {children}
    </article>
  </body>
</html>
</>
)
