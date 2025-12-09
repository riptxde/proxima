/**
 * Detects the Monaco editor language based on file name/extension
 * Falls back to Monaco's built-in language detection for most files
 * @param fileName - The name of the file (can include path)
 * @returns The Monaco editor language ID
 */
export function detectLanguage(fileName: string): string {
  // Extract just the filename from path if needed
  const name = fileName.split(/[\\/]/).pop() || fileName;

  // Handle special ignore files
  if (
    name === ".gitignore" ||
    name === ".dockerignore" ||
    name === ".proximaignore"
  ) {
    return "ini";
  }

  // Handle Lua-related files
  if (name.endsWith(".luau") || name.endsWith(".txt")) {
    return "lua";
  }

  // Extract extension for Monaco's built-in detection
  const ext = name.split(".").pop()?.toLowerCase();

  if (!ext) {
    return "lua";
  }

  // Map common extensions to Monaco languages
  // Monaco supports many languages out of the box
  const languageMap: Record<string, string> = {
    // Scripts
    js: "javascript",
    jsx: "javascript",
    ts: "typescript",
    tsx: "typescript",
    py: "python",
    rb: "ruby",
    php: "php",
    java: "java",
    cs: "csharp",
    cpp: "cpp",
    c: "c",
    h: "cpp",
    hpp: "cpp",
    go: "go",
    rs: "rust",
    swift: "swift",
    kt: "kotlin",
    scala: "scala",
    lua: "lua",

    // Web
    html: "html",
    htm: "html",
    xml: "xml",
    css: "css",
    scss: "scss",
    sass: "sass",
    less: "less",
    json: "json",
    yaml: "yaml",
    yml: "yaml",

    // Markup
    md: "markdown",
    markdown: "markdown",
    rst: "restructuredtext",
    tex: "latex",

    // Shell
    sh: "shell",
    bash: "shell",
    zsh: "shell",
    fish: "shell",
    ps1: "powershell",
    bat: "bat",
    cmd: "bat",

    // Config
    toml: "toml",
    ini: "ini",
    cfg: "ini",
    conf: "ini",
    properties: "properties",

    // Database
    sql: "sql",
    mysql: "mysql",
    pgsql: "pgsql",

    // Other
    r: "r",
    m: "objective-c",
    mm: "objective-c",
    dockerfile: "dockerfile",
    graphql: "graphql",
    proto: "protobuf",
    diff: "diff",
    patch: "diff",
    log: "log",
  };

  return languageMap[ext] || "lua";
}
