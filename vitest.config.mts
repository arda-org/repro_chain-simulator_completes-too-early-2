// eslint-disable-next-line import/no-unresolved
import { defineConfig } from "vitest/config";

export default defineConfig({
  esbuild: {
    target: "es2022",
  },
});
