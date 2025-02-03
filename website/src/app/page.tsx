"use client";

import { useEffect, useState } from "react";
import { 
  checkMasterPassword, 
  setMasterPassword, 
  verifyMasterPassword, 
  listPasswords, 
  addPassword, 
  deletePassword 
} from "@/app/utils/api";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Lock, Unlock, Eye, EyeOff, Search, Trash2 } from "lucide-react";
import { Sidebar } from "@/components/PasswordVault/Sidebar";
import { AddPasswordDialog } from "@/components/PasswordVault/AddPasswordDialog";

export default function Home() {
  const [isMasterSet, setIsMasterSet] = useState<boolean | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [masterPassword, setMasterPasswordInput] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [passwords, setPasswords] = useState<{ service: string; username: string; password: string }[]>([]);
  const [search, setSearch] = useState("");
  const [showPasswords, setShowPasswords] = useState(false);
  const [isAddPasswordOpen, setIsAddPasswordOpen] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        const needsSetup = await checkMasterPassword();
        console.log("[DEBUG] Needs master password setup:", needsSetup);
        setIsMasterSet(!needsSetup);
      } catch (error) {
        console.error("[ERROR] Failed to initialize:", error);
        setIsMasterSet(false);
      }
    })();
  }, []);

  const handleMasterSubmit = async () => {
    try {
      if (isMasterSet) {
        const success = await verifyMasterPassword(masterPassword);
        if (success) {
          setIsAuthenticated(true);
          await loadPasswords();
        } else {
          alert("Incorrect Master Password");
        }
      } else {
        if (masterPassword.length < 4) {
          alert("Master password must be at least 4 characters long");
          return;
        }
        if (masterPassword !== confirmPassword) {
          alert("Passwords do not match");
          return;
        }
        await setMasterPassword(masterPassword);
        setIsMasterSet(true);
        setIsAuthenticated(true);
        await loadPasswords();
      }
    } catch (error) {
      console.error("[ERROR] Failed to handle master password:", error);
      alert("An error occurred. Please try again.");
    }
  };

  const loadPasswords = async () => {
    try {
      const data = await listPasswords();
      console.log("[DEBUG] Loaded List_passwords:", data);
      setPasswords(data);
    } catch (error) {
      console.error("[ERROR] Failed to load passwords:", error);
    }
  };

  const handleAddPasswordSubmit = async (service: string, username: string, password: string) => {
    try {
      await addPassword(service, username, password);
      await loadPasswords();
    } catch (error) {
      console.error("[ERROR] Failed to add password:", error);
    }
  };

  const handleDeletePassword = async (service: string) => {
    try {
      await deletePassword(service);
      await loadPasswords();
    } catch (error) {
      console.error("[ERROR] Failed to delete password:", error);
    }
  };

  if (isMasterSet === null) {
    return (
      <div className="min-h-screen bg-[#1a0033] flex items-center justify-center p-4">
        <div className="text-white">Loading...</div>
      </div>
    );
  }

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-[#1a0033] flex items-center justify-center p-4">
        <Card className="w-full max-w-md bg-[#2d0059]/50 border-purple-500/20">
          <CardHeader className="text-center">
            <CardTitle className="text-3xl font-bold text-white">
              {isMasterSet ? (
                <div className="flex items-center justify-center gap-2">
                  <Lock className="w-8 h-8" /> Unlock Vault
                </div>
              ) : (
                <div className="flex items-center justify-center gap-2">
                  <Unlock className="w-8 h-8" /> Create Master Key
                </div>
              )}
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <input
              type="password"
              placeholder="Master Password"
              className="w-full px-4 py-2 rounded bg-[#3d007a]/30 border border-purple-500/20 text-white placeholder-purple-300/50 focus:outline-none focus:ring-2 focus:ring-purple-500/40"
              value={masterPassword}
              onChange={(e) => setMasterPasswordInput(e.target.value)}
            />
            {!isMasterSet && (
              <input
                type="password"
                placeholder="Confirm Master Password"
                className="w-full px-4 py-2 rounded bg-[#3d007a]/30 border border-purple-500/20 text-white placeholder-purple-300/50 focus:outline-none focus:ring-2 focus:ring-purple-500/40"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
              />
            )}
            <button 
              className="w-full py-2 rounded bg-purple-600 hover:bg-purple-700 text-white font-medium transition-colors"
              onClick={handleMasterSubmit}
            >
              {isMasterSet ? "Unlock" : "Set Password"}
            </button>
            {!isMasterSet && (
              <p className="text-sm text-purple-300/80 text-center">
                Password must be at least 4 characters long
              </p>
            )}
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen bg-[#1a0033]">
      <Sidebar onAddPassword={() => setIsAddPasswordOpen(true)} />
      
      <div className="flex-1 p-4 md:p-8">
        <div className="max-w-4xl mx-auto space-y-6">
          <div className="flex items-center gap-4">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-purple-300/50" />
              <input
                type="search"
                placeholder="Search passwords..."
                className="w-full pl-10 pr-4 py-2 rounded bg-[#3d007a]/30 border border-purple-500/20 text-white placeholder-purple-300/50 focus:outline-none focus:ring-2 focus:ring-purple-500/40"
                value={search}
                onChange={(e) => setSearch(e.target.value.trim())}
              />
            </div>
            <label className="flex items-center gap-2 text-white">
              <input
                type="checkbox"
                className="rounded border-purple-500/20 bg-[#3d007a]/30"
                checked={showPasswords}
                onChange={() => setShowPasswords(!showPasswords)}
              />
              <span className="flex items-center gap-1">
                {showPasswords ? <EyeOff size={16} /> : <Eye size={16} />}
                Show Passwords
              </span>
            </label>
          </div>

          <div className="space-y-2">
            {passwords
              .filter((item) => item.service.toLowerCase().includes(search.toLowerCase()))
              .map((item) => (
                <div key={item.service} className="p-4 rounded-lg bg-[#3d007a]/30 border border-purple-500/20 flex justify-between items-center">
                  <div className="text-white space-y-1">
                    <p className="font-medium">{item.service}</p>
                    <p className="text-purple-300/80 text-sm">{item.username}</p>
                    <p className="font-mono text-purple-200">{showPasswords ? item.password : "••••••••"}</p>
                  </div>
                  <button className="p-2 rounded hover:bg-red-500/20 text-red-400" onClick={() => handleDeletePassword(item.service)}>
                    <Trash2 size={20} />
                  </button>
                </div>
              ))}
          </div>
        </div>
      </div>

      <AddPasswordDialog isOpen={isAddPasswordOpen} onClose={() => setIsAddPasswordOpen(false)} onAdd={handleAddPasswordSubmit} />
    </div>
  );
}
