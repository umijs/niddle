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

test("should not change html structure", (t) => {
  const html = fs.readFileSync(path.resolve(__dirname, "jquery.html"), {
    encoding: "utf8",
  });

  const $ = parse(html);
  t.is(formatHtml($.outerHtml()), formatHtml(html));
});

test("should select attributes with ns correctly", (t) => {
  const $ = parse(`
<!DOCTYPE html>
<html>
<head>
  <title>HTML and SVG Namespace Example</title>
</head>
<body>
  <h1>This is an HTML Heading</h1>
  <p>This is an HTML paragraph.</p>
  <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
    <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
  </svg>
</body>
</html>
`);
  t.deepEqual(
    $.select("svg").getAttributes(),
    {
      xmlns: "http://www.w3.org/2000/svg",
      width: "100",
      height: "100",
    },
    "shoud select all attributes with ns correctly ",
  );

  t.is(
    $.select("svg").getAttribute("width"),
    "100",
    "shoud select single attribute with ns correctly ",
  );
});

test("shoud append child correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").append($2.select("#two"));

  t.is($1.selectAll(".one")[0].text(), "firstsecond");
  t.is($1.select(".one #two").text(), "second");
});

test("shoud prepend child correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").prepend($2.select("#two"));

  t.is($1.selectAll(".one")[0].text(), "secondfirst");
  t.is($1.select(".one #two").innerHtml(), "second");
});

test("shoud insert child after correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").insertAfter($2.select("#two"));

  t.deepEqual(
    $1.selectAll("div").map((e) => e.text()),
    ["first", "second"],
  );
});

test("shoud insert child before correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").insertBefore($2.select("#two"));

  t.deepEqual(
    $1.selectAll("div").map((e) => e.text()),
    ["second", "first"],
  );
});
