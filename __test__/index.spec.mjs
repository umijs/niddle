import test from "ava";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import beautify from "js-beautify";

import { parse } from "../index.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function formatHtml(html) {
  return beautify.html(html, { preserve_newlines: false });
}

const originHtml = fs.readFileSync(path.resolve(__dirname, "jquery.html"), {
  encoding: "utf8",
});

const $ = parse(originHtml);

test("should parse correctly", (t) => {
  t.is(formatHtml($.outerHtml()), formatHtml(originHtml));
});
