import axios from "axios";

const BASE_URL = "http://127.0.0.1:8080/api";
const MASTER_KEY_STORAGE_KEY = "master_key";

/**
 * Check if the master password is set.
 * @returns {Promise<boolean>} - True if the master password needs to be set, otherwise false.
 */
export const checkMasterPassword = async (): Promise<boolean> => {
  try {
    console.log("[DEBUG] Checking if master password exists...");
    const response = await axios.get(`${BASE_URL}/initialize`);

    console.log("[DEBUG] Master password check result:", response.data);

    if (typeof response.data === "string") {
      return !response.data.toLowerCase().includes("already set");
    }

    return response.data?.initialized === false;
  } catch (error) {
    console.error("[ERROR] Failed to check master password:", error);
    return false;
  }
};

/**
 * Set the master password.
 * @param {string} password - The master password to set.
 */
export const setMasterPassword = async (password: string): Promise<void> => {
  try {
    console.log("[DEBUG] Setting master password...");
    await axios.post(`${BASE_URL}/create-master-password`, { password });
    console.log("[DEBUG] Master password set successfully.");
  } catch (error) {
    console.error("[ERROR] Failed to set master password:", error);
  }
};

/**
 * Verify the master password.
 * @param {string} password - The master password to verify.
 * @returns {Promise<boolean>} - True if verification is successful, otherwise false.
 */
export const verifyMasterPassword = async (password: string): Promise<boolean> => {
  try {
    console.log("[DEBUG] Verifying master password...");
    const response = await axios.post(`${BASE_URL}/verify-master-password`, { password });

    console.log("[DEBUG] Master password verification result:", response.data);

    if (response.data?.key) {
      localStorage.setItem(MASTER_KEY_STORAGE_KEY, response.data.key);
      console.log("[DEBUG] Master key stored in local storage.");
      return true;
    }

    console.log("[DEBUG] Invalid master password.");
    return false;
  } catch (error) {
    console.error("[ERROR] Failed to verify master password:", error);
    return false;
  }
};

/**
 * Retrieve the stored master key from local storage.
 * @returns {string | null} - The master key or null if not found.
 */
export const getMasterKey = (): string | null => {
  return localStorage.getItem(MASTER_KEY_STORAGE_KEY);
};

/**
 * Fetch the list of stored passwords.
 * @returns {Promise<any[]>} - Array of password objects or an empty array on failure.
 */
export const listPasswords = async (): Promise<any[]> => {
  console.log("[DEBUG] Fetching password list...");

  const masterKey = getMasterKey();
  if (!masterKey) {
    console.warn("[WARNING] Master key not available. Please verify the master password first.");
    return [];
  }

  try {
    const response = await axios.get(`${BASE_URL}/list-passwords`);
    console.log("[DEBUG] Retrieved", response.data?.length || 0, "passwords.");
    return response.data;
  } catch (error) {
    console.error("[ERROR] Failed to fetch password list:", error);
    return [];
  }
};

/**
 * Add a new password.
 * @param {string} service - Service name.
 * @param {string} username - Username.
 * @param {string} password - Password.
 */
export const addPassword = async (service: string, username: string, password: string): Promise<void> => {
  const masterKey = getMasterKey();
  if (!masterKey) {
    console.warn("[WARNING] Cannot add password: Master key is missing.");
    return;
  }

  try {
    console.log(`[DEBUG] Adding password for service: ${service}...`);
    await axios.post(`${BASE_URL}/add-password`, { service, username, password });
    console.log("[DEBUG] Password added successfully.");
  } catch (error) {
    console.error("[ERROR] Failed to add password:", error);
  }
};

/**
 * Delete a stored password.
 * @param {string} service - Service name.
 */
export const deletePassword = async (service: string): Promise<void> => {
  const masterKey = getMasterKey();
  if (!masterKey) {
    console.warn("[WARNING] Cannot delete password: Master key is missing.");
    return;
  }

  try {
    console.log(`[DEBUG] Deleting password for service: ${service}...`);
    await axios.delete(`${BASE_URL}/remove-password/${service}`);
    console.log("[DEBUG] Password deleted successfully.");
  } catch (error) {
    console.error("[ERROR] Failed to delete password:", error);
  }
};

/**
 * Logout by clearing the master key.
 */
export const logout = (): void => {
  localStorage.removeItem(MASTER_KEY_STORAGE_KEY);
  console.log("[DEBUG] Master key removed from local storage.");
};
