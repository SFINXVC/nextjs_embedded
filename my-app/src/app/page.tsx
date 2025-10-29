import Image from "next/image";

// Force dynamic rendering (SSR) for this page
export const dynamic = 'force-dynamic';
export const revalidate = 0;

async function getServerData() {
  // This runs on the server at request time
  return {
    serverTime: new Date().toISOString(),
    renderType: 'Server-Side Rendered (SSR)',
    processId: process.pid,
  };
}

export default async function Home() {
  const serverData = await getServerData();
  
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex min-h-screen w-full max-w-3xl flex-col items-center justify-between py-32 px-16 bg-white dark:bg-black sm:items-start">
        <Image
          className="dark:invert"
          src="/next.svg"
          alt="Next.js logo"
          width={100}
          height={20}
          priority
        />
        <div className="flex flex-col items-center gap-6 text-center sm:items-start sm:text-left">
          <h1 className="max-w-xs text-3xl font-semibold leading-10 tracking-tight text-black dark:text-zinc-50">
            Next.js SSR Test for Rust Embedding
          </h1>
          
          {/* SSR Proof Section */}
          <div className="w-full max-w-md rounded-lg border border-zinc-200 dark:border-zinc-800 p-6 bg-zinc-50 dark:bg-zinc-900">
            <h2 className="text-xl font-semibold mb-4 text-black dark:text-zinc-50">
              SSR Status
            </h2>
            <div className="space-y-2 text-sm">
              <p className="text-zinc-600 dark:text-zinc-400">
                <span className="font-medium text-black dark:text-zinc-50">Render Type:</span>{" "}
                {serverData.renderType}
              </p>
              <p className="text-zinc-600 dark:text-zinc-400">
                <span className="font-medium text-black dark:text-zinc-50">Server Time:</span>{" "}
                {serverData.serverTime}
              </p>
              <p className="text-zinc-600 dark:text-zinc-400">
                <span className="font-medium text-black dark:text-zinc-50">Process ID:</span>{" "}
                {serverData.processId}
              </p>
              <p className="text-xs text-green-600 dark:text-green-400 mt-4">
                ✓ This page is fully server-side rendered on each request
              </p>
            </div>
          </div>
          
          <p className="max-w-md text-lg leading-8 text-zinc-600 dark:text-zinc-400">
            This Next.js app is configured for full SSR with standalone output mode,
            making it ready to be embedded in your Rust application.
          </p>
        </div>
        <div className="flex flex-col gap-4 text-base font-medium sm:flex-row">
          <a
            className="flex h-12 w-full items-center justify-center gap-2 rounded-full bg-foreground px-5 text-background transition-colors hover:bg-[#383838] dark:hover:bg-[#ccc] md:w-[158px]"
            href="https://nextjs.org/docs?utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            Documentation
          </a>
          <a
            className="flex h-12 w-full items-center justify-center rounded-full border border-solid border-black/[.08] px-5 transition-colors hover:border-transparent hover:bg-black/[.04] dark:border-white/[.145] dark:hover:bg-[#1a1a1a] md:w-[158px]"
            href="/api/time"
            target="_blank"
            rel="noopener noreferrer"
          >
            API Test
          </a>
        </div>
      </main>
    </div>
  );
}
