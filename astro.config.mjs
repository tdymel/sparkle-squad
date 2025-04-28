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
            sidebar: [
                {
                    label: "Positioning",
                    translations: {
                        "en": "Positioning",
                        "ru": "TODO",
                    },
                    items: [
                        {
                            label: "Defense",
                            translations: {
                                en: "Defense",
                                ru: ""
                            },
                            link: "https://sparkle-squad.de/docs/en/defense"
                        },
                    ],
                },
                {
                    label: "Match results",
                    translations: {
                        "en": "Match results",
                        "ru": "TODO",
                    },
                    link: "https://sparkle-squad.de/docs/en/history"
                },
            ],
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
        })
    ]
});