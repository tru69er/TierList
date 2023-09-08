# Tier List Desktop Website
A simple Tier List making desktop website. 

Use vim bindings (h,j,k,l) to navigate, then enter to select an item. Again, use vim bindings (j,k) to move the items up and down the tiers, and then enter to lock the tier and move to the next item. 

## Developing

Once you've cloned and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
