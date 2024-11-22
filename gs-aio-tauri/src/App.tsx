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
    <main className="min-h-screen bg-gray-900 text-gray-400">
      <section className="p-6">
        <h1 className="text-center align-middle text-4xl">Golden Sun Editor</h1>

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
      </section>
      <footer className="fixed bottom-0 flex w-full p-1">
        <span className="">v0.0.1</span>
      </footer>
    </main>
  );
}

export default App;
