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

test("should parse correctly", (t) => {
  const html = fs.readFileSync(path.resolve(__dirname, "jquery.html"), {
    encoding: "utf8",
  });

  const $ = parse(html);
  t.is(formatHtml($.outerHtml()), formatHtml(html));
});

test("should select attributes with ns correctly", (t) => {
  const html = fs.readFileSync(path.resolve(__dirname, "svg_ns.html"), {
    encoding: "utf8",
  });

  const $ = parse(html);
  t.deepEqual(
    $.querySelector("svg").getAttributes(),
    {
      xmlns: "http://www.w3.org/2000/svg",
      width: "100",
      height: "100",
    },
    "shoud select all attributes with ns correctly ",
  );

  t.is(
    $.querySelector("svg").getAttribute("width"),
    "100",
    "shoud select single attribute with ns correctly ",
  );
});
