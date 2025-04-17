import { defineConfig } from 'astro/config';
import sitemap from '@astrojs/sitemap';
import tailwind from "@astrojs/tailwind";

import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
    site: 'https://sparkle-squad.de',
    integrations: [
        sitemap(),
        tailwind({ applyBaseStyles: false }),
        starlight({
            title: "Sparkle Squad - Wiki",
            components: {
              LanguageSelect: './src/components/StarlightLanguageSelect.astro'
            },
            defaultLocale: 'de',
            locales: {
                de: {
                    label: "Deutsch",
                },
                en: {
                    label: "English",
                },
                ru: {
                    label: "Русский",
                },
            },
            sidebar: [
                {
                    label: "Getting started",
                    items: [
                        {
                            label: "TODO",
                            link: "/TODO",
                        },
                    ],
                },
            ]
        })
    ]
});