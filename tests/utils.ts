import { Keypair } from "@solana/web3.js";
import { readFileSync } from "fs";
import path from "path";

/**
 * Helper functions.
 */

function createKeypairFromFile(path: string): Keypair {
  return Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(path, "utf-8")))
  );
}

export const user = createKeypairFromFile(
  path.resolve(__dirname, "./user.json")
);
export const user2 = createKeypairFromFile(
  path.resolve(__dirname, "./user_2.json")
);
