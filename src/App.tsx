import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import { commands, type Profiles } from "./bindings";
import Database from "@tauri-apps/plugin-sql";
// when using `"withGlobalTauri": true`, you may use
// const Database = window.__TAURI__.sql;

function App() {
  const [profiles, setProfiles] = useState<Profiles[]>([]);
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");

  async function createProfile() {
    // const db = await Database.load("sqlite:password_manager.db");
    // await db.execute(
    //   'INSERT INTO `profiles` (`id`, `name`, `created_at`, `pass_hash`) VALUES (?, ?, ?, ?) -- binds: ["AxC6ucXvWOcwID1p4C", "nithin", 2025-11-19T09:48:08.658668, "$2b$12$CKHT.QI62jB/jn5Z6/2IbeXLw51hnSV79EMAv4.piLZXjzZK4k362"]'
    // );
    // if (!name || !password) {
    //   setMessage("Please enter both name and password");
    //   return;
    // }
    let x = await commands.createProfile("ntihin", "sdfdsaf");

    // const result = await commands.createProfile(name, password);

    setMessage(JSON.stringify(x));
    setName("");
    setPassword("");
    // Refresh the list after creating
  }

  async function loadProfiles() {
    const result = await commands.listProfile();
    // const db = await Database.load("sqlite:password_manager.db");

    // let result = await db.select("select * from profiles;");

    // setProfiles(result);
    setMessage(`Loaded ${JSON.stringify(result)}  profile(s)`);
  }

  return (
    <main className="container">
      <h1>Password Manager</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <div style={{ marginTop: "2rem" }}>
        <h2>Create Profile</h2>
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            createProfile();
          }}
        >
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter profile name..."
          />
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.currentTarget.value)}
            placeholder="Enter password..."
          />
          <button type="submit">Create Profile</button>
        </form>
      </div>

      <div style={{ marginTop: "2rem" }}>
        <h2>Profiles</h2>
        <button onClick={loadProfiles}>Load Profiles</button>

        {profiles.length > 0 && (
          <div style={{ marginTop: "1rem" }}>
            <table style={{ width: "100%", borderCollapse: "collapse" }}>
              <thead>
                <tr>
                  <th style={{ border: "1px solid #ccc", padding: "8px" }}>
                    ID
                  </th>
                  <th style={{ border: "1px solid #ccc", padding: "8px" }}>
                    Name
                  </th>
                  <th style={{ border: "1px solid #ccc", padding: "8px" }}>
                    Created At
                  </th>
                </tr>
              </thead>
              <tbody>
                {profiles.map((profile) => (
                  <tr key={profile.id}>
                    <td style={{ border: "1px solid #ccc", padding: "8px" }}>
                      {profile.id}
                    </td>
                    <td style={{ border: "1px solid #ccc", padding: "8px" }}>
                      {profile.name}
                    </td>
                    <td style={{ border: "1px solid #ccc", padding: "8px" }}>
                      {new Date(profile.created_at).toLocaleString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {message && <p style={{ marginTop: "1rem", color: "#0f0" }}>{message}</p>}
    </main>
  );
}

export default App;
