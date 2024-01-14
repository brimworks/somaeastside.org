const title = "(Dev) Soma Eastside Church";

import Header from './_includes/templates/Header.tsx'

// ')] scale-75 w-full bg-center bg-cover"></img>"
export default ({ }: Lume.Data, helpers: Lume.Helpers) => (
  <>
<html>
  <head>
    <title>{ title }</title>
    <meta charSet="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="stylesheet" href="/index.css" />
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto" />
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto:300" />
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto:400" />
    <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Public+Sans:900" />
  </head>
  <body className="dark:bg-black">
    <Header />
    <div className="relative w-full">
      <div className="sticky top-0 overflow-hidden w-full h-90 flex z-10">
        <img src="/static/img/john-price-RAZQiZOX3mU-unsplash.jpg" className="w-full scale-110 self-center translate-y-2" />
      </div>
      <div className="relative backdrop-brightness-75 bg-white/30 z-20">
        <div className="container mx-auto p-20">
            <h1 className="text-2xl sm:text-4xl md:text-5xl font-black font-banner">MAKING DISCIPLES<br/>
            WHO MAKE DISCIPLES
            </h1>
            <p className="max-w-prose">We exist to help people follow Jesus and encourage them to show his saving love, power, and presence to the Eastside.</p>
        </div>
      </div>
    </div>
    <div className="bg-gray-800 p-10">
      <div className="flex flex-wrap justify-center gap-8">
        <img src="/static/img/thomas-speckhardt.jpg" className="w-60 h-80 object-cover" />
        <div className="text-gray-400 w-96">
          <blockquote className="text-gray-300">
            “I have the privilege of working with values-driven CEOs who have the audacity to believe they can bend the future to their will.
            If you want to build something that lasts, be proud of how you got there, and become a better version of yourself, I would love to help.”
          </blockquote>
          <br />
          <footer className="font-light">
            CHRIS HARE,<br />
            FOUNDER & PRINCIPAL<br />
            <cite className="font-normal">THE STORIED FUTURE</cite>
          </footer>
        </div>
      </div>
      <div className="h-screen">&nbsp;</div>
    </div>
  </body>
</html>
</>
)
