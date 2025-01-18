import { FormEvent } from "react";

export default function App() {
  const handleSubmit = async(e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const formData = new FormData(e.currentTarget);
    const inputValue = formData.get("input-section");

    try {
      const response = await fetch("YOUR_API_ENDPOINT", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({message: inputValue}),
      });

      if(!response.ok) {
        throw new Error("Network response was not ok.");
      }

      const data = await response.json();
      console.log(data);
    } catch(error) {
      console.error("Error: ", error);
    }
  };
  
  return (
    <div className="bg-black h-screen w-full flex items-center justify-center font-outfit text-sm">
      <form onSubmit={handleSubmit} className="relative w-3/5 h-[5%]">
        <div className="w-full h-full">
          <label htmlFor="input-section"></label>
          <input 
            type="text" 
            name="input-section" 
            id="input-section" 
            placeholder="write here..." 
            autoComplete="off"
            className="px-4 inline-flex items-center bg-button-white rounded-full w-full h-full outline-none"
          />
        </div>
          <div className="absolute -right-1 top-0 h-full w-1/6 bg-button-black hover:bg-stroke-black rounded-full border-2 border-stroke-black text-white flex items-center justify-center hover:cursor-pointer">
            <input type="submit" name="send-button" id="send-button" value={"send"} className="hover:cursor-pointer" />
          </div>
      </form>
    </div>
  );
}