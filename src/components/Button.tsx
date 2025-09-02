"use client";
import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type MyComponentPage = {
  children: React.ReactNode;
};

export default function Button({ children }: MyComponentPage) {
  async function handle_click() {
    let data: string = await invoke("enforce_leetcode");
    setMessage(data);
    setDisplayGreeting(() => {
      true;
    });
  }
  const [displayGreeting, setDisplayGreeting] = useState(() => {
    false;
  });
  const [message, setMessage] = useState("");

  return (
    <div>
      <button
        onClick={() => handle_click()}
        className="py-2 px-6 font-semibold text-white bg-red-600 rounded-lg shadow-md transition duration-200 hover:bg-red-700 hover:shadow-lg focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:outline-none active:bg-red-800"
      >
        {" "}
        {children}
      </button>
      {displayGreeting ?? <div>{message}</div>}
    </div>
  );
}
