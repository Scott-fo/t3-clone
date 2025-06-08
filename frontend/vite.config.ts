import path from "path";
import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import tsconfigPaths from "vite-tsconfig-paths";
import { reactRouter } from "@react-router/dev/vite";

export default defineConfig(({ command }) => {
  const isBuild = command === "build";

  return {
    plugins: [tailwindcss(), reactRouter(), tsconfigPaths()],

    resolve: {
      alias: [
        { find: "@", replacement: path.resolve(__dirname, "./") },
        ...(isBuild
          ? [
              {
                find: "react-dom/server",
                replacement: "react-dom/server.node",
              },
            ]
          : []),
      ],
    },

    server: {
      proxy: {
        "/api": {
          target: "http://localhost:8080",
          changeOrigin: true,
        },
      },
    },
  };
});
