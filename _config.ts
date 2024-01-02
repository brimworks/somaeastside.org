import lume from "lume/mod.ts";
import esbuild from "lume/plugins/esbuild.ts";
import jsx from "lume/plugins/jsx.ts";
import feed from "lume/plugins/feed.ts";
import minify_html from "lume/plugins/minify_html.ts";
import nav from "lume/plugins/nav.ts";
import nunjucks from "lume/plugins/nunjucks.ts";
import pagefind from "lume/plugins/pagefind.ts";
import relative_urls from "lume/plugins/relative_urls.ts";
import resolve_urls from "lume/plugins/resolve_urls.ts";
import sass from "lume/plugins/sass.ts";
import sitemap from "lume/plugins/sitemap.ts";
import slugify_urls from "lume/plugins/slugify_urls.ts";
import source_maps from "lume/plugins/source_maps.ts";
import tailwindcss from "lume/plugins/tailwindcss.ts";
import postcss from "lume/plugins/postcss.ts";
import typography from "npm:@tailwindcss/typography";

const site = lume({
    src: "./src",
});
site.use(esbuild({
    extensions: [".browser.tsx", ".js"],
}));
site.use(jsx({
    extensions: [".site.tsx"],
}));
site.use(feed());
site.use(minify_html());
site.use(nav());
site.use(nunjucks());
site.use(pagefind());
site.use(relative_urls());
site.use(resolve_urls());
site.use(sass());
site.use(sitemap());
site.use(slugify_urls());
site.use(source_maps());
site.use(tailwindcss({
    extensions: [".html", ".js"],
    // tailwind.config.js:
    options: {
        theme: {
            fontFamily: {
                sans: ['Raleway', 'sans-serif'],
            },
        },
        plugins: [
            typography,
        ]
    },
}));
site.use(postcss());

site.copy("static");
site.copy("favicon.ico");
site.ignore("README.md");


export default site;
