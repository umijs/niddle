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
  const jqueryHtml = fs.readFileSync(path.resolve(__dirname, "jquery.html"), {
    encoding: "utf8",
  });
  const $ = parse(jqueryHtml);
  t.is(formatHtml($.outerHtml()), formatHtml(jqueryHtml));
});

test("should select node and get attributes with ns correctly", (t) => {
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

  t.is(
    $1.select(".one").outerHtml(),
    '<div class="one">first<div id="two">second</div></div>',
  );
});

test("shoud prepend child correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").prepend($2.select("#two"));

  t.is(
    $1.select(".one").outerHtml(),
    '<div class="one"><div id="two">second</div>first</div>',
  );
});

test("shoud insert child after correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div><div>three</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").insertAfter($2.select("#two"));

  t.is(
    $1.select("body").outerHtml(),
    '<body><div class="one">first</div><div id="two">second</div><div>three</div></body>',
  );
});

test("shoud insert child before correctly", (t) => {
  const $1 = parse(
    '<html><head></head><body><div class="one">first</div><div>three</div></body></html>',
  );
  const $2 = parse(
    '<html><head></head><body><div id="two">second</div></body></html>',
  );

  $1.select(".one").insertBefore($2.select("#two"));

  t.is(
    $1.select("body").outerHtml(),
    '<body><div id="two">second</div><div class="one">first</div><div>three</div></body>',
  );
});

test("shoud set attribute correctly", (t) => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );

  $.select(".one").setAttribute("id", "Hello");

  t.deepEqual($.select(".one").getAttribute("id"), "Hello");
});

test("shoud set attributes correctly", (t) => {
  const $ = parse(
    '<html><head></head><body><div class="one">first</div></body></html>',
  );

  $.select(".one").setAttributes({ id: "Hello", data: "Niddle" });

  t.deepEqual($.select(".one").getAttributes(), {
    class: "one",
    id: "Hello",
    data: "Niddle",
  });
});
