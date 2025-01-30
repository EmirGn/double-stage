import "./App.css";
import Sidebar from "./components/Sidebar";
import Welcome from "./components/Welcome";
import Chat from "./components/Chat";

import { useState } from "react";
import { useParams, useNavigate } from "react-router";

export default function App() {
  const params = useParams();
  const navigate = useNavigate();
  const [responseText, setResponseText] = useState("");

  const handlePostData = async (inputValue: string) => {
    try {
      const response = await fetch("http://127.0.0.1:5000/gemini/", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: inputValue,
      });
      if (!response.ok) {
        throw new Error("Network response was not ok.");
      }
      const data = await response.text();
      setResponseText(data);
    } catch (error) {
      console.error("Error: ", error);
    }
  };

  return (
    <div className="flex flex-row w-screen bg-extra_black">
      <div className="flex-none w-[280px]">
        <Sidebar />
      </div>
      <div className="flex-1">
        {params.id ? (
          <Chat />
        ) : (
          <Welcome clickHandler={handlePostData} />
        )}
      </div>
    </div>
  );
}
