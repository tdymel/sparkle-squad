import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';
import { defineCollection, z } from 'astro:content';

const blogSchema = z.object({
    identifier: z.string(),
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.string().optional(),
    heroImage: z.string().optional(),
    badge: z.string().optional(),
});

const blogCollection = defineCollection({ schema: blogSchema });

export const collections = {
    news: blogCollection,
    docs: defineCollection({ loader: docsLoader(), schema: docsSchema() }),
};