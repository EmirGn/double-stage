import { useState } from "react";
import { Link } from "react-router";
export default function Sidebar() {
  const [latestChat, setLatestChat] = useState<string[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const handleClick = () => {
    setLatestChat([...latestChat, `Chat ${currentIndex + 1}`]);
    setCurrentIndex(currentIndex + 1);
  };
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
            {latestChat.map((chat, index) => (
              <Link
                key={index}
                to={`/${index + 1}`}
                className="p-2 mx-2 rounded-lg hover:bg-text_black"
              >
                {chat}
              </Link>
            ))}
          </div>
        </div>
      </nav>
    </div>
  );
}
