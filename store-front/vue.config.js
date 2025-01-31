const { defineConfig } = require('@vue/cli-service');

module.exports = defineConfig({
  transpileDependencies: true,

  // Configure the development server
  devServer: {
    proxy: {
      // Define proxy rules
      '/api': {
        target: 'http://20.151.164.146:3000', // Proxy all requests starting with /api to the backend service on port 3000
        changeOrigin: true,  // This is needed for virtual hosted sites
        pathRewrite: { '^/api': '' }, // Rewrite the /api on requests to ''
      }
    }
  }
});
