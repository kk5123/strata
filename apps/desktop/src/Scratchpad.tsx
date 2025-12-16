import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import "./Scratchpad.css";

export function Scratchpad() {
  const [text, setText] = useState("");
  const win = getCurrentWindow();

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        close();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const close = async () => {
    await invoke("scratchpad_closed").catch(console.error);
    await win.hide().catch(console.error);
  };

  return (
    <div className="scratch-root">
      <h2 className="scratch-title">Scratchpad</h2>
      <textarea
        className="scratch-textarea"
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="いま考えていることをメモ..."
        autoFocus
      />
      <div className="scratch-actions">
        <button onClick={close}>閉じる</button>
      </div>
    </div>
  );
}
