import lume from "lume/mod.ts";
import esbuild from "lume/plugins/esbuild.ts";
import jsx from "lume/plugins/jsx_preact.ts";
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
import liquid from "lume/plugins/liquid.ts";

// Markdown plugin configuration
const markdown = {
    extensions: [".md", ".markdown", ".html"]
};

const site = lume({
    src: "./src",
}, { markdown });
site.use(esbuild({
    extensions: [".js"],
}));
site.use(jsx({
    extensions: [".jsx", ".tsx"],
}));
site.use(liquid({
    options: {
        extname: ".liquid"
    }
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
                sans: ['Roboto', 'sans-serif'],
                banner: ['Public Sans', 'sans-serif']
            },
            screens: {
                'sm': '640px',
                'md': '1024px',
                'lg': '1280px',
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
