import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const src = readFileSync(join(root, "src/tape.html"), "utf8");
const out = join(root, "dist");

const grab = (open, close) => {
  const a = src.indexOf(open), b = src.indexOf(close, a);
  if (a < 0 || b < 0) throw new Error(`missing ${open} in src/tape.html`);
  return src.slice(a + open.length, b);
};
const css = grab("<style>", "</style>");
const js = grab("<script>", "</script>");
const head = src.slice(src.indexOf("<head>") + 6, src.indexOf("<style>"));
const body = src.slice(src.indexOf("<body>") + 6, src.indexOf("<script>"));

rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "app.css"), css.trim());
writeFileSync(join(out, "app.js"), js.trim());
writeFileSync(join(out, "index.html"), `<!doctype html>
<html lang="en"><head>${head}<link rel="stylesheet" href="app.css">
</head><body>${body}<script src="app.js"></script>
</body></html>`);
console.log(`dist: index.html + app.css (${(css.length / 1024) | 0} KB) + app.js (${(js.length / 1024) | 0} KB)`);
