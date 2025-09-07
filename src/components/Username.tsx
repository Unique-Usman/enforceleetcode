"use client";

import { useState } from "react";
// import { get_user_name } from "@tauri-apps/api/core";

export default function LeetcodeForm() {
  const [username, setUsername] = useState("");
  const [message, setMessage] = useState("");

  const submitUsername = async () => {
    if (!username) {
      setMessage("❌ Please enter a username.");
      return;
    }
    try {
      // const result = await get_user_name("save_and_check_username", {
      //   username,
      // });

      const result = "true";
      setMessage(result);
    } catch (err) {
      setMessage("❌ Error: " + err);
    }
  };

  return (
    <div className="flex flex-col justify-center items-center p-4 min-h-screen bg-gray-100">
      <div className="p-8 w-full max-w-sm bg-white rounded-lg shadow-md">
        <h1 className="mb-6 text-2xl font-bold text-center text-gray-800">
          LeetCode Enforcer
        </h1>

        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Enter your LeetCode username"
          className="py-2 px-4 mb-4 w-full rounded-lg border border-gray-300 focus:border-transparent focus:ring-2 focus:ring-blue-500 focus:outline-none"
        />

        <button
          onClick={submitUsername}
          className="py-2 w-full font-semibold text-white bg-blue-500 rounded-lg transition-colors duration-200 hover:bg-blue-600"
        >
          Save & Check
        </button>

        {message && (
          <p
            className={`mt-4 text-center font-medium ${message.includes("✅") ? "text-green-600" : "text-red-600"
              }`}
          >
            {message}
          </p>
        )}
      </div>
    </div>
  );
}
