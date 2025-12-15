import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "./Scratchpad.css";

export function Scratchpad() {
  const [text, setText] = useState("");
  const win = getCurrentWindow();

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        win.hide().catch(console.error);
      }
      if (e.key === "Enter" && e.ctrlKey) {
        e.preventDefault();
        submit();
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const submit = async () => {
    // TODO: 保存やAI呼び出しをここに実装
    console.log("scratch:", text);
    setText("");
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
        <button onClick={() => win.hide().catch(console.error)}>閉じる</button>
      </div>
    </div>
  );
}
