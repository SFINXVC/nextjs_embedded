import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Enable standalone output for embedding with Rust
  output: 'standalone',
  
  // Disable static optimization to force SSR
  experimental: {
    // This ensures the app runs as a server
  },
};

export default nextConfig;
