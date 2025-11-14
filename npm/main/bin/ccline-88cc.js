#!/usr/bin/env node
const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

const isTermux = Boolean(
  process.env.TERMUX_VERSION ||
  (process.env.PREFIX && process.env.PREFIX.includes('/com.termux'))
);

// 1. Priority: Use user-provided binary inside ~/.claude/ccline if present
const claudeHome = path.join(os.homedir(), '.claude', 'ccline');
const claudeCandidates = [
  path.join(claudeHome, process.platform === 'win32' ? 'ccline.exe' : 'ccline'),
  path.join(claudeHome, process.platform === 'win32' ? 'ccline-88cc.exe' : 'ccline-88cc')
];

const claudePath = claudeCandidates.find(candidate => fs.existsSync(candidate));
if (claudePath) {
  const result = spawnSync(claudePath, process.argv.slice(2), {
    stdio: 'inherit',
    shell: false
  });
  process.exit(result.status || 0);
}

// 2. Fallback: Use npm package binary
const platform = process.platform;
const arch = process.arch;

// Handle special cases
let platformKey = `${platform}-${arch}`;
if (platform === 'linux') {
  // Detect if static linking is needed based on glibc version
  function shouldUseStaticBinary() {
    try {
      const { execSync } = require('child_process');
      const lddOutput = execSync('ldd --version 2>/dev/null || echo ""', { 
        encoding: 'utf8',
        timeout: 1000 
      });
      
      // Parse "ldd (GNU libc) 2.35" format
      const match = lddOutput.match(/(?:GNU libc|GLIBC).*?(\d+)\.(\d+)/);
      if (match) {
        const major = parseInt(match[1]);
        const minor = parseInt(match[2]);
        // Use static binary if glibc < 2.35
        return major < 2 || (major === 2 && minor < 35);
      }
    } catch (e) {
      // If detection fails, default to dynamic binary
      return false;
    }
    
    return false;
  }
  
  if (arch === 'x64' && shouldUseStaticBinary()) {
    platformKey = 'linux-x64-musl';
  }

  if (arch === 'arm64') {
    platformKey = 'linux-arm64';
  }
}

const packageMap = {
  'darwin-x64': '@gary-50/ccline-88cc-darwin-x64',
  'darwin-arm64': '@gary-50/ccline-88cc-darwin-arm64',
  'linux-x64': '@gary-50/ccline-88cc-linux-x64',
  'linux-x64-musl': '@gary-50/ccline-88cc-linux-x64-musl',
  'linux-arm64': '@gary-50/ccline-88cc-linux-arm64',
  'win32-x64': '@gary-50/ccline-88cc-win32-x64',
  'win32-ia32': '@gary-50/ccline-88cc-win32-x64', // Use 64-bit for 32-bit systems
};

const packageName = packageMap[platformKey];
if (!packageName) {
  console.error(`Error: Unsupported platform ${platformKey}`);
  if (isTermux) {
    console.error('Detected Termux environment. Please run `cargo build --release` and copy the binary to ~/.claude/ccline/ccline-88cc.');
  } else {
    console.error('Supported platforms: darwin (x64/arm64), linux (x64/arm64), win32 (x64)');
    console.error('Please visit https://github.com/gary-50/CCometixLine_termux for manual installation or building from source.');
  }
  process.exit(1);
}

const binaryName = platform === 'win32' ? 'ccline.exe' : 'ccline';
const binaryPath = path.join(__dirname, '..', 'node_modules', packageName, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(`Error: Binary not found at ${binaryPath}`);
  console.error('This might indicate a failed installation or unsupported platform.');
  console.error('Please try reinstalling: npm install -g @gary-50/ccline-88cc');
  console.error(`Expected package: ${packageName}`);
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  shell: false
});

process.exit(result.status || 0);
