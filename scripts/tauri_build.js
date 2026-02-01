import {
  existsSync,
  mkdirSync,
  copyFileSync,
  readdirSync,
  statSync,
  rmSync,
} from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";
import archiver from "archiver";
import { createWriteStream } from "fs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, "..");

// Paths
const distTauriDir = join(rootDir, "dist-tauri");
const proximaDir = join(distTauriDir, "proxima");
const scriptsDir = join(proximaDir, "scripts");
const autoexecDir = join(proximaDir, "autoexec");
const releaseDir = join(rootDir, "src-tauri", "target", "release");
const bundleDir = join(releaseDir, "bundle");
const nsisDir = join(bundleDir, "nsis");
const exePath = join(releaseDir, "proxima.exe");
const zipPath = join(distTauriDir, "proxima.zip");

function cleanBuildDirectories() {
  // Clean dist-tauri
  if (existsSync(distTauriDir)) {
    console.log("Cleaning existing dist-tauri directory...");
    rmSync(distTauriDir, { recursive: true, force: true });
  }

  // Clean bundle directory to prevent old artifacts
  if (existsSync(bundleDir)) {
    console.log("Cleaning existing bundle directory...");
    rmSync(bundleDir, { recursive: true, force: true });
  }
}

function createDirectories() {
  console.log("Creating distribution directories...");
  mkdirSync(distTauriDir, { recursive: true });
  mkdirSync(proximaDir, { recursive: true });
  mkdirSync(scriptsDir, { recursive: true });
  mkdirSync(autoexecDir, { recursive: true });
}

function copyExe() {
  if (!existsSync(exePath)) {
    throw new Error(`proxima.exe not found at ${exePath}`);
  }

  console.log("Copying proxima.exe...");
  copyFileSync(exePath, join(distTauriDir, "proxima.exe"));
}

function copyNsisFiles() {
  if (!existsSync(nsisDir)) {
    throw new Error(`NSIS bundle directory not found at ${nsisDir}`);
  }

  console.log("Copying NSIS bundle files...");
  const files = readdirSync(nsisDir);

  for (const file of files) {
    const srcPath = join(nsisDir, file);
    const destPath = join(proximaDir, file);

    if (statSync(srcPath).isFile()) {
      copyFileSync(srcPath, destPath);
    } else {
      // Copy directory recursively
      copyDirectoryRecursive(srcPath, destPath);
    }
  }

  console.log(`Copied ${files.length} items from NSIS bundle`);
}

function copyDirectoryRecursive(src, dest) {
  mkdirSync(dest, { recursive: true });
  const entries = readdirSync(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = join(src, entry.name);
    const destPath = join(dest, entry.name);

    if (entry.isDirectory()) {
      copyDirectoryRecursive(srcPath, destPath);
    } else {
      copyFileSync(srcPath, destPath);
    }
  }
}

function createZip() {
  return new Promise((resolve, reject) => {
    console.log("Creating proxima.zip...");

    const output = createWriteStream(zipPath);
    const archive = archiver("zip", {
      zlib: { level: 9 }, // Maximum compression
    });

    output.on("close", () => {
      const sizeInMB = (archive.pointer() / 1024 / 1024).toFixed(2);
      console.log(`Zip created: ${sizeInMB} MB`);
      resolve();
    });

    archive.on("error", (err) => {
      reject(err);
    });

    archive.pipe(output);
    archive.directory(proximaDir, "proxima");
    archive.finalize();
  });
}

async function main() {
  try {
    // Step 1: Clean build directories
    cleanBuildDirectories();

    // Step 2: Run Tauri build
    console.log("Starting Tauri build...");
    console.log("Running: tauri build");
    execSync("tauri build", {
      stdio: "inherit",
      cwd: rootDir,
    });

    console.log("\nTauri build completed successfully\n");

    // Step 3: Create distribution structure
    createDirectories();

    // Step 4: Copy files
    copyExe();
    copyNsisFiles();

    // Step 5: Create zip
    await createZip();

    console.log("\nDistribution package created successfully");
    console.log(`Location: ${distTauriDir}`);
    console.log(`Zip file: ${zipPath}`);
  } catch (error) {
    console.error("\nBuild failed:");
    console.error(error.message);
    process.exit(1);
  }
}

main();
