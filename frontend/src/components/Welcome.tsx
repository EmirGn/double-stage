import { useState } from "react";

interface WelcomeProps {
  clickHandler: (inputValue: string) => void;
}

export default function Welcome({ clickHandler }: WelcomeProps) {
  const [inputValue, setInputValue] = useState('');

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value);
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
        <button onClick={() => clickHandler(inputValue)} className="absolute bg-white rounded-full text-black px-4 py-2 bottom-2 right-2 ">send</button>
      </div>

    </div>
  );
}