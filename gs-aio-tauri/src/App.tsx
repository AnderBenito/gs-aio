import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

function App() {
  const [filePath, setFilePath] = useState<string>("");

  const handleFileLoad = async () => {
    try {
      // Open the file dialog to select a file
      const selectedFilePath = await open({
        multiple: false,
        filters: [
          { name: "Text Files", extensions: ["gba"] },
          { name: "All Files", extensions: ["*"] },
        ],
      });

      if (selectedFilePath) {
        setFilePath(selectedFilePath);
        invoke("load_rom", { filePath: selectedFilePath });
      }
    } catch (error) {
      console.error("Error loading file:", error);
    }
  };

  return (
    <main className="bg-gray-900 min-h-screen text-gray-400 p-6">
      <h1 className="align-middle text-center  text-4xl">Golden Sun Editor</h1>

      <Button variant="secondary" onClick={handleFileLoad}>
        Load File
      </Button>
      <Input />
      {filePath && (
        <div>
          <p>
            <strong>File Path:</strong> {filePath}
          </p>
        </div>
      )}
    </main>
  );
}

export default App;
