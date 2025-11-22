import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import { commands, type Profiles } from "./bindings";

function App() {
  const [profiles, setProfiles] = useState<Profiles[]>([]);
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");

  async function createProfile() {
    if (!name || !password) {
      setMessage("Please enter both name and password");
      return;
    }
    setMessage("Plcreateind");

    const result = await commands.createProfile(name, password);

    if (result.status === "ok") {
      setMessage("Profile created successfully!");
      setName("");
      setPassword("");
      // Refresh the list after creating
      await loadProfiles();
    } else {
      setMessage(`Error: ${result.error}`);
    }
  }

  async function loadProfiles() {
    const result = await commands.listProfile();

    if (result.status === "ok") {
      setProfiles(result.data);
      setMessage(`Loaded ${result.data.length} profile(s)`);
    } else {
      setMessage(`Error loading profiles: ${result.error}`);
    }
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
