import { z, defineCollection } from "astro:content";
const blogSchema = z.object({
    identifier: z.string(),
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    updatedDate: z.string().optional(),
    heroImage: z.string().optional(),
    badge: z.string().optional(),
});

export type BlogSchema = z.infer<typeof blogSchema>;

const blogCollection = defineCollection({ schema: blogSchema });

export const collections = {
    'news': blogCollection,
}