import { useState, useEffect } from "react";
import { Link } from "react-router";
import { v4 as uuidv4 } from "uuid";

interface Chat {
  id: string
  name: string
}

export default function Sidebar() {
  const [latestChat, setLatestChat] = useState<Chat[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  
  const handleClick = () => {
    const newChat = {
      id: uuidv4(),
      name: `Chat ${currentIndex + 1}`,
    }

    setLatestChat([...latestChat, newChat]);
    setCurrentIndex(currentIndex + 1);
  };

  useEffect(() => {
    const fetchChats = async () => {
      try {
        const response = await fetch('http://127.0.0.1:5000/c');
        const data = await response.json();
        setLatestChat(data);
        setCurrentIndex(data.length);
      } catch (error) {
        console.error('Error fetching chats:', error);
      }
    };

    fetchChats();
  }, []);

  return (
    <div className="w-[280px]">
      <nav
        className="flex flex-col h-screen p-6 bg-black text-white rounded-r-2xl font-outfit"
      >
        <div className="flex flex-col gap-2">
          <Link to={"/"}>
            <p className="text-2xl">
              double stage
            </p>
          </Link>
          <button
            onClick={handleClick}
            className="self-start py-2 px-4 bg-white text-text_black rounded-2xl outline-none"
          >
            new chat
          </button>
        </div>
        <hr
          className="my-6 border-t border-white opacity-30"
        />
        <div
          className="flex flex-col gap-8 h-full overflow-auto"
        >
          <p className="text-xl">
            latest chats:
          </p>
          <div className="flex flex-col gap-4">
            {latestChat.map((chat) => (
              <Link
                key={chat.id}
                to={`/c/${chat.id}`}
                className="p-2 mx-2 rounded-lg hover:bg-text_black"
              >
                {chat.name}
              </Link>
            ))}
          </div>
        </div>
      </nav>
    </div>
  );
}
