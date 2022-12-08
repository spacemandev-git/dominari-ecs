/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  swcMinify: true,
  webpack: (config, options) => {
    config.experiments = {
      asyncWebAssembly: true
    }
    return config;
  }
}

module.exports = nextConfig
