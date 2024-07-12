import test from "ava";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import beautify from "js-beautify";

import { parse } from "../index.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const originHtml = beautify.html(
  fs.readFileSync(path.resolve(__dirname, "jquery.html"), {
    encoding: "utf8",
  }),
  {
    preserve_newlines: false,
  },
);

const domTree = parse(originHtml);

test("should parse correctly", (t) => {
  t.deepEqual(
    beautify.html(domTree.outerHtml(), { preserve_newlines: false }),
    originHtml,
  );
});
