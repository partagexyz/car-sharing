/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  webpack: (config, { dev }) => {
    config.cache = false; //disable cache to avoid caching issues
    if (dev) {
      config.devtool = 'cheap-module-source-map'; //enable source maps for debugging
    }
    return config;
  },
};

module.exports = nextConfig;
