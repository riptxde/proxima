import { createHash } from "crypto";
import { readFileSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

/**
 * Recursively get all files in a directory
 */
function getAllFiles(dirPath, arrayOfFiles = []) {
  const files = readdirSync(dirPath);

  files.forEach((file) => {
    const filePath = join(dirPath, file);
    const stat = statSync(filePath);

    if (stat.isDirectory()) {
      arrayOfFiles = getAllFiles(filePath, arrayOfFiles);
    } else {
      arrayOfFiles.push(filePath);
    }
  });

  return arrayOfFiles;
}

/**
 * Calculate hash of file contents
 */
function hashFile(filePath) {
  const content = readFileSync(filePath);
  return createHash("sha256").update(content).digest("hex");
}

/**
 * Calculate recursive hash of all files in directories and specific files
 */
export function calculateProjectHash(rootDir, directories, files) {
  const hash = createHash("sha256");
  const allPaths = [];

  // Add all files from directories
  directories.forEach((dir) => {
    const dirPath = join(rootDir, dir);
    try {
      const dirFiles = getAllFiles(dirPath);
      allPaths.push(...dirFiles);
    } catch (err) {
      // Directory might not exist, skip
    }
  });

  // Add specific files
  files.forEach((file) => {
    const filePath = join(rootDir, file);
    try {
      if (statSync(filePath).isFile()) {
        allPaths.push(filePath);
      }
    } catch (err) {
      // File might not exist, skip
    }
  });

  // Sort paths for consistent ordering
  allPaths.sort();

  // Hash each file and combine
  allPaths.forEach((filePath) => {
    const relPath = relative(rootDir, filePath);
    const fileHash = hashFile(filePath);

    // Include both path and hash to detect renames
    hash.update(relPath);
    hash.update(fileHash);
  });

  return hash.digest("hex");
}
