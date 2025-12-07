import { readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";
import { calculateProjectHash } from "./hash_util.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, "..");

// Directories to watch
const WATCH_DIRS = ["src", "public"];

// Specific files to watch
const WATCH_FILES = [
  "components.json",
  "index.html",
  "package.json",
  "package-lock.json",
  "tsconfig.json",
  "tsconfig.app.json",
  "tsconfig.node.json",
  "vite.config.ts",
];

const HASH_FILE = ".vue_build_hash";

function main() {
  console.log("Calculating project hash...");

  // Calculate current hash
  const currentHash = calculateProjectHash(rootDir, WATCH_DIRS, WATCH_FILES);

  // Check if hash file exists
  const hashFilePath = join(rootDir, HASH_FILE);
  let shouldBuild = true;

  if (existsSync(hashFilePath)) {
    const previousHash = readFileSync(hashFilePath, "utf-8").trim();

    if (previousHash === currentHash) {
      console.log("No changes detected, skipping build");
      shouldBuild = false;
    } else {
      console.log("Changes detected, rebuilding...");
    }
  } else {
    console.log("No previous build found, building...");
  }

  if (shouldBuild) {
    try {
      // Run the actual build command
      console.log("Running: vue-tsc -b && vite build");
      execSync("vue-tsc -b && vite build", {
        stdio: "inherit",
        cwd: rootDir,
      });

      // Save the new hash
      writeFileSync(hashFilePath, currentHash);
      console.log("Build completed successfully");
      console.log(`Hash saved: ${currentHash.substring(0, 12)}...`);
    } catch (error) {
      console.error("Build failed");
      process.exit(1);
    }
  }
}

main();
