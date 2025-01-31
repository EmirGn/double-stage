import { v4 as uuidv4 } from "uuid";

import { useState } from "react";
import { useNavigate } from "react-router";

interface WelcomeProps {
  clickHandler: (inputValue: string, chat_id: string) => void;
}

export default function Welcome({ clickHandler }: WelcomeProps) {
  const [inputValue, setInputValue] = useState('');
  const navigate = useNavigate();

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value);
  };

  const handleSubmit = () => {
    const newChatId = uuidv4();
    navigate(`/${newChatId}`);
    clickHandler(inputValue, newChatId);
  };

  return (
    <div className="w-full h-screen flex flex-col gap-8 justify-center items-center font-outfit text-white">
      <h1 className="text-2xl">
        Welcome to Double Stage!
      </h1>

      <div className="relative bg-black w-[800px] h-[80px] rounded-xl">
        <input 
          type="text"
          placeholder="message double-stage"
          value={inputValue}
          onChange={handleInputChange}
          className="bg-black w-[800px] h-[80px] rounded-xl text-left pb-8 px-4 text-white hover:bg-zinc-900 outline-none"
        />
        <button onClick={handleSubmit} className="absolute bg-white rounded-full text-black px-4 py-2 bottom-2 right-2 ">send</button>
      </div>

    </div>
  );
}