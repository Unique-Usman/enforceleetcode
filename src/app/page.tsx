import Image from "next/image";
import Button from "@/components/Button";

export default function Home() {
  return (
    <div className="grid gap-16 justify-items-center items-center p-8 pb-20 min-h-screen font-sans sm:p-20 grid-rows-[20px_1fr_20px]">
      <main className="flex flex-col row-start-2 items-center align-middle sm:items-center gap-[32px]">
        <h1>
          Welcome to Enforce Leetcode - Do LeetCode or get your system Shutdown.
        </h1>
        <Button>Participate in the Challenge</Button>
      </main>
      <footer className="flex flex-wrap row-start-3 justify-center items-center gap-[24px]"></footer>
    </div>
  );
}
