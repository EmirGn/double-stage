import "./App.css";
import Sidebar from "./components/Sidebar";
import Welcome from "./components/Welcome";
import Chat from "./components/Chat";

// import { useState } from "react";
import { useParams } from "react-router";

export default function App() {
  const params = useParams();
  // const [responseText, setResponseText] = useState("");

  const handlePostData = async (input_value: string, chat_id: string) => {
    try {
      const payload = { input_value, chat_id };
      const response = await fetch("http://127.0.0.1:5000/c", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      });
      if (!response.ok) {
        throw new Error("Network response was not ok.");
      }
      const data = await response.text();
      console.log(data);
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
