import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { BrowserRouter, Routes, Route } from "react-router";

import App from "./App.tsx";
// import Chat from "./components/Chat.tsx";
// import Sidebar from "./components/Sidebar.tsx";
// import Layout from "./components/Layout.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App/>}/>
      </Routes>
    </BrowserRouter>
  </StrictMode>,
);
