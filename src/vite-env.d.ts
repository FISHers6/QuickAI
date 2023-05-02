/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module 'vue3-markdown';
declare module 'vue3-highlightjs';

import * as ES6Promise from 'es6-promise';
ES6Promise.polyfill();