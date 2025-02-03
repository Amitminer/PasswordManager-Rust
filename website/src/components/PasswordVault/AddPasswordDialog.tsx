import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useState, ChangeEvent } from "react";

interface AddPasswordDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onAdd: (website: string, username: string, password: string) => void;
}

export function AddPasswordDialog({ isOpen, onClose, onAdd }: AddPasswordDialogProps) {
  const [website, setWebsite] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onAdd(website, username, password);
    setWebsite("");
    setUsername("");
    setPassword("");
    onClose();
  };

  const handleWebsiteChange = (e: ChangeEvent<HTMLInputElement>) => setWebsite(e.target.value);
  const handleUsernameChange = (e: ChangeEvent<HTMLInputElement>) => setUsername(e.target.value);
  const handlePasswordChange = (e: ChangeEvent<HTMLInputElement>) => setPassword(e.target.value);

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="bg-[#2d0059] border-purple-500/20 text-white">
        <DialogHeader>
          <DialogTitle>Add New Password</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Input
              placeholder="Website"
              value={website}
              onChange={handleWebsiteChange}
              className="bg-[#3d007a]/30 border-purple-500/20 text-white placeholder-purple-300/50"
            />
          </div>
          <div className="space-y-2">
            <Input
              placeholder="Username"
              value={username}
              onChange={handleUsernameChange}
              className="bg-[#3d007a]/30 border-purple-500/20 text-white placeholder-purple-300/50"
            />
          </div>
          <div className="space-y-2">
            <Input
              type="password"
              placeholder="Password"
              value={password}
              onChange={handlePasswordChange}
              className="bg-[#3d007a]/30 border-purple-500/20 text-white placeholder-purple-300/50"
            />
          </div>
          <div className="flex justify-end gap-2">
            <Button
              type="button"
              variant="ghost"
              onClick={onClose}
              className="text-purple-300 hover:text-purple-200 hover:bg-purple-500/20"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              className="bg-purple-600 hover:bg-purple-700 text-white"
            >
              Add Password
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
} 