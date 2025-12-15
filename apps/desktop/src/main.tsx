import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { Scratchpad } from "./Scratchpad";
import { HashRouter, Routes, Route } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <HashRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/quick" element={<Scratchpad />} />
      </Routes>
    </HashRouter>
  </React.StrictMode>,
);
